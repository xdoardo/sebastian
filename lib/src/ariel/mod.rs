use self::{
    mware::{http::HttpArielMiddleware, ArielMiddleware},
    page::{ArielPage, ArielPageData, ArielTitlePage},
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

    pub async fn login(&mut self) -> anyhow::Result<()> {
        self.middleware.login().await
    }

    pub async fn search(&mut self, course_name: &str) -> anyhow::Result<Vec<ArielTitlePage>> {
        log::info!("passing '{}' to middleware", course_name);
        self.middleware.search(course_name).await
    }

    pub async fn page_from_url(&mut self, url: String) -> anyhow::Result<ArielPage> {
        let (url, raw) = self.middleware.get(url.clone()).await?;
        log::debug!("making page from raw for url {}", url);
        ArielPage::from_raw(raw, url)
    }

    pub async fn get_children(&mut self, page: ArielPage) -> Vec<ArielPage> {
        let children_urls = page.get_children();
        if children_urls.len() == 1 && children_urls[0] == format!("{}v5", page.url.clone()) {
            let url = children_urls[0].clone();
            if let Ok((url, raw)) = self.middleware.get(url).await {
                log::debug!("making page from raw for url {}", url);
                if let Ok(page) = ArielPage::from_raw(raw, url) {
                    return vec![page];
                }
                return vec![];
            }
        }

        log::info!("got urls {:?} for page {}", children_urls, page.url);
        let mut res = vec![];
        for url in children_urls {
            if let Ok((url, raw)) = self.middleware.get(url.clone()).await {
                log::debug!("making page from raw for url {}", url);
                if let Ok(page) = ArielPage::from_raw(raw, url) {
                    res.push(page);
                }
            }
        }
        res
    }

    pub async fn download<'a>(
        &mut self,
        path: String,
        data: ArielPageData,
        chunk_done_size_chan: std::sync::mpsc::Sender<u64>,
    ) -> anyhow::Result<()> {
        self.middleware
            .download(path, data, chunk_done_size_chan)
            .await
    }

    pub async fn get_size<'a>(&mut self, data: &'a ArielPageData) -> anyhow::Result<u64> {
        self.middleware.get_size(data).await
    }
}
