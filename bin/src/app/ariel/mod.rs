mod login;
mod scrape;
mod search;

use super::{AppConfig, CURRENT_DIR};
use lazy_static::lazy_static;
use sebastian_core::{
    ariel::ArielUserConfig,
    ariel::{map::ArielSitemap, ArielNavigator},
};

lazy_static! {
    static ref OUTPUT_DIR: String = {
        let mut c = CURRENT_DIR.clone();
        c.push_str("/result");
        c
    };
    static ref ARIEL_SITEMAP: ArielSitemap = ArielSitemap::default();
}

/// Access the Ariel website and search for content to scrape.
#[derive(clap::Parser, Debug)]
pub(crate) struct Ariel {
    #[clap(subcommand)]
    pub action: ArielAction,

    #[clap(skip)]
    pub user_config: Option<ArielUserConfig>,

    #[clap(skip)]
    pub app_config: Option<AppConfig>,

    #[clap(skip)]
    nav: Option<ArielNavigator>,
}

#[derive(clap::Parser, Clone, Debug)]
pub(crate) enum ArielAction {
    Scrape(scrape::Scrape),
    Init(login::Login),
    Search(search::Search),
}

impl Ariel {
    pub(crate) async fn run(
        &mut self,
        app_config: AppConfig,
        user_config: Option<ArielUserConfig>,
    ) -> anyhow::Result<(AppConfig, ArielUserConfig)> {
        log::trace!(
            "app_config: {:?}, user_config: {:?}",
            app_config,
            user_config
        );
        self.user_config = user_config;
        self.app_config = Some(app_config);
        log::debug!("app: {:?}", self);

        match self.action {
            ArielAction::Scrape(scrape::Scrape {
                auto,
                ref output,
                ref url,
            }) => {
                let output = output.clone();
                let url = url.clone();

                if let Some(cfg) = &self.user_config {
                    self.nav = Some(ArielNavigator::new(cfg.clone()));
                    self.nav.as_mut().unwrap().login().await?
                } else {
                    self.login(None, None, false).await?;
                };
                self.scrape(auto, output, url).await?
            }
            ArielAction::Init(login::Login {
                ref username,
                ref password,
            }) => {
                let username = if username.is_none() && self.user_config.is_some() {
                    Some(self.user_config.as_ref().unwrap().clone().username)
                } else {
                    None
                };
                self.login(username, password.clone(), false).await?;
                self.app_config.as_mut().unwrap().save = true;
            }

            ArielAction::Search(search::Search { ref name }) => {
                let name = name.clone();
                if let Some(cfg) = &self.user_config {
                    self.nav = Some(ArielNavigator::new(cfg.clone()));
                    self.nav.as_mut().unwrap().login().await?
                } else {
                    self.login(None, None, false).await?;
                };
                self.search(name).await?
            }
        };

        Ok((
            self.app_config.as_ref().unwrap().clone(),
            self.user_config.as_ref().unwrap().clone(),
        ))
    }
}
