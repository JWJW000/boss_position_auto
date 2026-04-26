use super::*;

impl<'a> Poster<'a> {
    /// Fill the job-requirements section based on the selected recruitment type.
    pub(super) fn fill_requirements_by_type(&mut self, job: &JobRecord) -> BResult<()> {
        let kind = RecruitmentKind::parse(&job.招聘类型)?;
        log::info!("  [DEBUG] 职位要求分支: {}", kind.label());

        match kind {
            RecruitmentKind::FullTime => self.fill_full_time_requirements(job, kind),
            RecruitmentKind::Campus => self.fill_campus_requirements(job, kind),
            RecruitmentKind::Intern => self.fill_intern_requirements(job, kind),
            RecruitmentKind::PartTime => self.fill_part_time_requirements(job, kind),
        }?;

        Ok(())
    }
}
