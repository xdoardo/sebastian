use super::Ariel;
use crate::app::CURRENT_DIR;
use sebastian_core::ariel::{
    map::ArielSitemap,
    page::{ArielPage, ArielPageData},
    ArielNavigator,
};

lazy_static::lazy_static! {
    static ref OUTPUT_DIR: String = {
        let mut c = CURRENT_DIR.clone();
        c.push_str("/result");
        c
    };
    static ref ARIEL_SITEMAP: ArielSitemap = ArielSitemap::default();
}

/// Perform scraping.
#[derive(clap::Parser, Clone, Debug)]
pub(crate) struct Scrape {
    /// Perform scraping of Ariel website interactively.
    #[clap(short, long)]
    pub auto: bool,

    /// The base of the directory to save the results.
    #[clap(short, long, default_value = &OUTPUT_DIR)]
    pub output: String,

    /// The URL of the page to start the scraping from.
    #[clap(default_value = &ARIEL_SITEMAP.home_page_url)]
    pub url: String,
}

impl Ariel {
    pub(crate) fn scrape(&mut self, auto: bool, output: String, url: String) -> anyhow::Result<()> {
        let page = self.nav.as_mut().unwrap().page_from_url(url)?;
        log::debug!("page: {:?}", page);
        let mut to_ask = page.get_data();
        let mut stack = self.nav.as_mut().unwrap().get_children(page);

        while stack.len() != 0 {
            let child_page = stack.pop().unwrap();
            to_ask.append(&mut child_page.get_data());
            stack.append(&mut self.nav.as_mut().unwrap().get_children(child_page))
        }

        let select = inquire::MultiSelect::new("Select data to scrape: ", to_ask);

        Ok(())
    }
}
