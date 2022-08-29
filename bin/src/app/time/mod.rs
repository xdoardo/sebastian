use sebastian_core::time::TimeTableConfig;

use super::AppConfig;

/// Access your course's timetable.
#[derive(clap::Parser, Clone, Debug)]
pub struct Time {
    #[clap(subcommand)]
    pub action: TimeTableAction,

    #[clap(skip)]
    pub user_config: Option<TimeTableConfig>,
}

#[derive(clap::Parser, Clone, Debug)]
pub enum TimeTableAction {
    /// Initialize the timetable configuration.
    Init,

    /// Show the timetable.
    Show,
}

impl Time {
    pub(crate)  fn run(
        &mut self,
        app_config: AppConfig,
        user_config: Option<TimeTableConfig>
    ) -> anyhow::Result<(AppConfig, TimeTableConfig)> {
        todo!()
    }
}
