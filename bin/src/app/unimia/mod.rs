use sebastian_core::unimia::UnimiaUserConfig;

use super::AppConfig;

/// Access UniMia and show your personal informations.
#[derive(clap::Parser, Clone, Debug)]
pub struct Unimia {
    #[clap(subcommand)]
    pub action: UnimiaAction,

    #[clap(skip)]
    pub user_config: Option<UnimiaUserConfig>,
}

#[derive(clap::Parser, Clone, Debug)]
pub enum UnimiaAction {
    /// Search unimia.
    Show,
    /// Initialize unimia.
    Init,
}

impl Unimia {
    pub(crate) async fn run(
        &mut self,
        app_config: AppConfig,
        user_config: Option<UnimiaUserConfig>,
    ) -> anyhow::Result<(AppConfig, UnimiaUserConfig)> {
        todo!()
    }
}
