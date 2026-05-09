//! BOSS直聘自动化发布岗位 - 主程序入口
//!
//! 用法:
//!   boss_auto                        # 读取同目录Excel，扫码登录，发布全部职位
//!   boss_auto --excel path.xlsx      # 指定Excel路径
//!   boss_auto --dry-run              # 只读Excel不发布
//!   boss_auto --relogin              # 强制重新扫码

use boss_auto::{
    export_failed_jobs, utils::sleep_random_ms, BossClient, ExcelReader, FailedJob, JobRecord,
    Poster,
};
use log::{error, info, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "boss_auto", about = "BOSS直聘自动化发布岗位工具")]
struct Args {
    /// Excel文件路径（默认：exe同目录下的.xlsx文件）
    #[structopt(short, long)]
    excel: Option<PathBuf>,

    /// 跳过发布，仅读取并显示Excel内容
    #[structopt(long)]
    dry_run: bool,

    /// 强制重新扫码登录（忽略已有cookie）
    #[structopt(long)]
    relogin: bool,

    /// 从第N行开始发布（1-based，用于断点续传）
    #[structopt(short, long, default_value = "1")]
    start_row: usize,
}

/// Initialize terminal and file logging before any work starts.
fn setup_logging() {
    let log_file = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("boss_auto.log")))
        .unwrap_or_else(|| PathBuf::from("boss_auto.log"));

    let config = ConfigBuilder::new().build();

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .unwrap_or_else(|e| panic!("无法打开日志文件 {:?}: {}", log_file, e));

    let mut loggers: Vec<Box<dyn SharedLogger>> =
        vec![WriteLogger::new(LevelFilter::Info, config.clone(), file)];
    let term = TermLogger::new(
        LevelFilter::Info,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    loggers.push(term);
    CombinedLogger::init(loggers).expect("初始化日志系统失败");

    info!("BOSS直聘自动化发布工具启动");
    info!("日志文件: {:?}", log_file);
}

/// Find the first `.xlsx` file next to the executable.
fn find_excel() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))?;

    let entries = fs::read_dir(&exe_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
            return Some(path);
        }
    }
    None
}

/// Keep the console open so the user can inspect final logs.
fn wait_for_exit() {
    print!("\n执行结束，按 Enter 关闭窗口...");
    io::stdout().flush().ok();
    let _ = io::stdin().read_line(&mut String::new());
}

/// Execute the CLI workflow and return the process exit code.
fn run() -> ExitCode {
    let args = Args::from_args();

    // 确定Excel路径
    let excel_path = match args.excel {
        Some(p) => p,
        None => match find_excel() {
            Some(p) => p,
            None => {
                error!("未找到Excel文件，请使用 --excel 指定路径");
                return ExitCode::from(1);
            }
        },
    };

    info!("使用的Excel: {:?}", excel_path);

    // 读取Excel
    let reader = ExcelReader::new(&excel_path);
    let jobs: Vec<JobRecord> = match reader.read() {
        Ok(jobs) => jobs,
        Err(e) => {
            error!("读取Excel失败: {}", e);
            return ExitCode::from(1);
        }
    };

    info!("共读取 {} 条职位信息", jobs.len());

    if args.dry_run {
        for (i, job) in jobs.iter().enumerate() {
            println!("\n--- 职位 {}: {} ---", i + 1, job.职位名称);
            println!("  类型: {}", job.招聘类型);
            println!("  经验: {}", job.经验);
            println!("  城市: {}", job.城市);
            println!("  薪资: {}-{} {}", job.薪资低, job.薪资高, job.薪资单位);
            println!("  结算方式: {}", job.结算方式);
            println!("  学历: {}", job.学历);
            println!("  关键词: {}", job.关键词);
        }
        return ExitCode::SUCCESS;
    }

    // 过滤到起始行
    let jobs: Vec<_> = jobs.into_iter().skip(args.start_row - 1).collect();
    if jobs.is_empty() {
        error!("没有需要发布的职位（起始行 {} > 总行数）", args.start_row);
        return ExitCode::from(1);
    }
    info!("从第{}条开始发布，共 {} 条", args.start_row, jobs.len());

    // 初始化BOSS客户端
    let mut client = match BossClient::new(args.relogin) {
        Ok(c) => c,
        Err(e) => {
            error!("初始化失败: {}", e);
            return ExitCode::from(1);
        }
    };

    // 扫码登录（如果需要）
    if let Err(e) = client.login_if_needed() {
        error!("登录失败: {}", e);
        return ExitCode::from(1);
    }

    // 发布职位。单条失败仅记录并继续，便于定位每一步的问题。
    let mut poster = Poster::new(&mut client);
    let mut success = 0;
    let mut failed = 0;
    let mut failed_jobs: Vec<FailedJob> = Vec::new();

    for (index, job) in jobs.iter().enumerate() {
        let row_number = args.start_row + index;
        match poster.post(job) {
            Ok(url) => {
                success += 1;
                info!("[成功] {} -> {}", job.职位名称, url);
                if index + 1 < jobs.len() {
                    info!("发布成功，等待 2.5-4 秒后继续下一个岗位...");
                    sleep_random_ms(25_00, 40_00);
                }
            }
            Err(e) => {
                failed += 1;
                let error_msg = format!("{}", e);
                error!("[失败] 第{}条 {}: {}", row_number, job.职位名称, error_msg);

                // 记录失败的岗位
                failed_jobs.push(FailedJob {
                    row_number,
                    job: job.clone(),
                    error_message: error_msg,
                });
            }
        }
    }

    info!("\n===== 发布完成 =====");
    info!("成功: {} | 失败: {}", success, failed);

    // 如果有失败的岗位，导出到 Excel
    if !failed_jobs.is_empty() {
        let failed_excel_path = excel_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("失败岗位记录.xlsx");

        match export_failed_jobs(&failed_jobs, &failed_excel_path) {
            Ok(_) => {
                info!("失败岗位已导出到: {:?}", failed_excel_path);
            }
            Err(e) => {
                error!("导出失败岗位Excel失败: {}", e);
            }
        }
    }

    if failed > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// Program entry point.
fn main() -> ExitCode {
    setup_logging();
    let code = run();
    wait_for_exit();
    code
}
