use self::{
    mware::{http::HttpArielMiddleware, ArielMiddleware},
    page::{ArielPage, ArielTitlePage},
};

pub mod map;
pub mod mware;
pub mod page;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArielUserConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct ArielNavigator {
    middleware: Box<dyn ArielMiddleware>,
}

impl ArielNavigator {
    pub fn new(config: ArielUserConfig) -> Self {
        ArielNavigator {
            middleware: Box::new(HttpArielMiddleware::new(config)),
        }
    }

    pub fn login(&mut self) -> anyhow::Result<()> {
        self.middleware.login()
    }

    pub fn search(&mut self, course_name: &str) -> anyhow::Result<Vec<ArielTitlePage>> {
        log::info!("passing '{}' to middleware", course_name);
        self.middleware.search(course_name)
    }

    pub fn page_from_url(&mut self, url: String) -> anyhow::Result<ArielPage> {
        let (raw, url) = self.middleware.get(url.clone(), true)?;
        log::debug!("making page from raw for url {}", url);
        Ok(ArielPage::from_raw(raw, url))
    }

    pub fn get_children(&mut self, page: ArielPage) -> Vec<ArielPage> {
        let children_urls = page.get_children();
        log::debug!("got urls {:?} for page {:?}", children_urls, page);
        let mut res = vec![];
        for url in children_urls {
            if let Ok((raw, url)) = self.middleware.get(url.clone(), false) {
                log::debug!("making page from raw for url {}", url);
                res.push(ArielPage::from_raw(raw, url));
            }
        }
        res
    }
}
