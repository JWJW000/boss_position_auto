//! 错误类型定义

use rust_drission::CdpError;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BossError {
    #[error("Excel读取错误: {0}")]
    Excel(String),

    #[error("BOSS网站错误: {0}")]
    BossSite(String),

    #[error("元素未找到: {0}")]
    ElementNotFound(String),

    #[error("登录超时")]
    LoginTimeout,

    #[error("Cookie已过期，请重新扫码登录")]
    CookieExpired,

    #[error("发布失败: {0}")]
    PostFailed(String),

    #[error("配置文件错误: {0}")]
    Config(String),

    #[error("CDP错误[{context}]: {source}")]
    Cdp {
        context: String,
        #[source]
        source: CdpError,
    },

    #[error("网络错误: {0}")]
    Network(String),
}

impl BossError {
    /// Build an ElementNotFound error from a plain message.
    pub fn element(message: impl Into<String>) -> Self {
        Self::ElementNotFound(message.into())
    }

    /// Build an ElementNotFound error with a context prefix.
    pub fn element_ctx(context: impl AsRef<str>, source: impl Display) -> Self {
        Self::ElementNotFound(format!("{}: {}", context.as_ref(), source))
    }

    /// Build a PostFailed error with a context prefix.
    pub fn post_ctx(context: impl AsRef<str>, source: impl Display) -> Self {
        Self::PostFailed(format!("{}: {}", context.as_ref(), source))
    }

    /// Build a Config error with a context prefix.
    pub fn config_ctx(context: impl AsRef<str>, source: impl Display) -> Self {
        Self::Config(format!("{}: {}", context.as_ref(), source))
    }

    /// Preserve rust_drission's native CDP exception as the source error.
    pub fn cdp(context: impl Into<String>, source: CdpError) -> Self {
        Self::Cdp {
            context: context.into(),
            source,
        }
    }

    /// Build a `map_err` mapper for ElementNotFound with context.
    pub fn map_element<E: Display>(context: impl Into<String>) -> impl FnOnce(E) -> Self {
        let context = context.into();
        move |e| Self::element_ctx(&context, e)
    }

    /// Build a `map_err` mapper for PostFailed with context.
    pub fn map_post<E: Display>(context: impl Into<String>) -> impl FnOnce(E) -> Self {
        let context = context.into();
        move |e| Self::post_ctx(&context, e)
    }

    /// Build a `map_err` mapper for Config with context.
    pub fn map_config<E: Display>(context: impl Into<String>) -> impl FnOnce(E) -> Self {
        let context = context.into();
        move |e| Self::config_ctx(&context, e)
    }

    /// Build a `map_err` mapper for rust_drission CDP errors.
    pub fn map_cdp(context: impl Into<String>) -> impl FnOnce(CdpError) -> Self {
        let context = context.into();
        move |e| Self::cdp(&context, e)
    }
}

impl From<CdpError> for BossError {
    fn from(source: CdpError) -> Self {
        BossError::cdp("CDP调用失败", source)
    }
}

pub type Result<T> = std::result::Result<T, BossError>;
