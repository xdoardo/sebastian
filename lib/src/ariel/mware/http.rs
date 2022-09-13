use std::path::PathBuf;

use async_trait::async_trait;
use reqwest::Client;

use super::ArielMiddleware;
use crate::ariel::{
    map::ArielSitemap,
    page::{ArielLoginPage, ArielPageData, ArielTitlePage},
    ArielUserConfig,
};

#[derive(Debug)]
pub struct HttpArielMiddleware {
    config: ArielUserConfig,
    sitemap: ArielSitemap,
    cookies: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
    client: Client,
}

#[async_trait]
impl ArielMiddleware for HttpArielMiddleware {
    fn new(config: ArielUserConfig) -> Self
    where
        Self: Sized,
    {
        let cookies = std::sync::Arc::new(reqwest_cookie_store::CookieStoreMutex::default());
        let user_agent = String::from("Sebastian");
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(cookies.clone())
            .user_agent(user_agent)
            .redirect(reqwest::redirect::Policy::limited(15))
            .build()
            .unwrap();
        HttpArielMiddleware {
            config,
            sitemap: ArielSitemap::default(),
            cookies,
            client,
        }
    }

    async fn login(&mut self) -> anyhow::Result<()> {
        log::info!("logging in...");
        let (_, text) = self
            .post(
                self.sitemap.login_url.clone(),
                vec![
                    ("hdnSilent".into(), "true".into()),
                    ("tbLogin".into(), self.config.username.clone()),
                    ("tbPassword".into(), self.config.password.clone()),
                ],
            )
            .await?;

        ArielLoginPage::is_logged_in(text)
    }

    async fn search(&mut self, course_name: &str) -> anyhow::Result<Vec<ArielTitlePage>> {
        let res = self
            .post(
                self.sitemap.search_url.clone(),
                vec![("keyword".into(), course_name.into())],
            )
            .await?;
        Ok(crate::ariel::page::ArielSearchPage::title_pages(res.1))
    }

    async fn get(&mut self, url: String) -> anyhow::Result<(String, String)> {
        self.get(url).await
    }

    async fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
    ) -> anyhow::Result<(String, String)> {
        self.post(url, form).await
    }

    async fn is_logged_in(&mut self) -> anyhow::Result<()> {
        let (_, page) = self.get(self.sitemap.home_page_url.clone()).await?;
        crate::ariel::page::ArielLoginPage::is_logged_in(page)
    }

    async fn download<'a>(
        &mut self,
        path: String,
        data: ArielPageData,
        chunk_done_size_chan: std::sync::mpsc::Sender<u64>,
    ) -> anyhow::Result<()> {
        match data.kind {
            crate::ariel::page::ArielPageDataKind::LessonStream => {
                self.download_stream(path, data, chunk_done_size_chan).await
            }
            crate::ariel::page::ArielPageDataKind::Generic => {
                self.download_generic(path, data, chunk_done_size_chan)
                    .await
            }
        }
    }

    async fn get_size<'a>(&mut self, data: &'a ArielPageData) -> anyhow::Result<u64> {
        match data.kind {
            crate::ariel::page::ArielPageDataKind::LessonStream => {
                self.get_size_of_stream(data.url.clone()).await
            }
            crate::ariel::page::ArielPageDataKind::Generic => {
                self.get_size_generic(data.url.clone()).await
            }
        }
    }
}

impl HttpArielMiddleware {
    #[async_recursion::async_recursion]
    async fn get(&mut self, url: String) -> anyhow::Result<(String, String)> {
        let res = self.client.get(url.clone()).send().await?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("posting to url '{}', status {}", url, status)
        }

        let url = res.url().clone();
        let text = res.text().await?;

        if text.contains(r#"<META HTTP-EQUIV="REFRESH" CONTENT="0; URL=v5">"#) {
            let url = url.join("v5")?.to_string();
            return self.get(url).await;
        }

        log::debug!("{} --- {}", url, text);
        Ok((url.to_string(), text))
    }

    #[async_recursion::async_recursion]
    async fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
    ) -> anyhow::Result<(String, String)> {
        log::info!("{:?}", self.cookies);

        let req = self.client.post(url.clone()).form(&form);
        log::info!("{:?}", req);
        let res = req.send().await?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("posting to url '{}', status {}", url, status)
        }

        let url = res.url().clone();
        let text = res.text().await?;

        if text.contains(r#"<META HTTP-EQUIV="REFRESH" CONTENT="0; URL=v5">"#) {
            let url = url.join("v5")?.to_string();
            return self.post(url, form).await;
        }

        log::debug!("{} --- {}", url, text);
        Ok((url.to_string(), text))
    }

    pub(crate) async fn get_bytes(&mut self, url: String) -> anyhow::Result<bytes::Bytes> {
        let res = self.client.get(url.clone()).send().await?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("posting to url '{}', status {}", url, status)
        }

        Ok(res.bytes().await?)
    }

    async fn get_size_generic(&mut self, url: reqwest::Url) -> anyhow::Result<u64> {
        let res = self.client.head(url.clone()).send().await?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("HEAD to url '{}', status {}", url, status)
        }

        if let Some(size) = res.headers().get("Content-Length") {
            if let Ok(size) = size.to_str()?.parse::<u64>() {
                return Ok(size);
            }
        }

        log::warn!("Could not find Content-Length!");
        Ok(0)
    }

    async fn get_size_of_stream(&mut self, _: reqwest::Url) -> anyhow::Result<u64> {
        //    All this is way too slow. One day maybe...
        //    let chunks = self.get_m3u8_segments(url.clone()).await?;
        //    let mut bytes = 0;
        //    for chunk in chunks {
        //        println!("getting size of chunk");
        //        bytes += self
        //            .get_size_generic(chunk.uri.parse::<url::Url>().unwrap())
        //            .await?;
        //    }
        //    Ok(bytes)
        Ok(0)
    }

    async fn download_stream(
        &mut self,
        path: String,
        data: ArielPageData,
        chunk_done_size_chan: std::sync::mpsc::Sender<u64>,
    ) -> anyhow::Result<()> {
        let chunks = self.get_m3u8_segments(data.url.clone()).await?;

        let mut path = PathBuf::from(path);
        path.push(heck::AsSnakeCase(data.from_site.clone()).to_string());
        path.push(heck::AsSnakeCase(data.from_ambient.clone()).to_string());
        path.push(heck::AsSnakeCase(data.from_thread.clone()).to_string());
        path.push(data.get_name());

        if !path.exists() {
            log::trace!("creating path {:?}", path);
            std::fs::create_dir_all(path.parent().unwrap())?
        }

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut bytes;
        for chunk in chunks {
            bytes = self.get_bytes(chunk.uri).await?;

            while ArielLoginPage::is_login_page_raw(&bytes) {
                self.login().await?;
                bytes = self.get_bytes(data.url.to_string()).await?;
            }

            let len = bytes.len();

            std::io::Write::write_all(&mut file, &bytes)?;
            chunk_done_size_chan.send(len.try_into().unwrap())?;
        }
        drop(chunk_done_size_chan);
        return Ok(());
    }

    async fn download_generic(
        &mut self,
        path: String,
        data: ArielPageData,
        chunk_done_size_chan: std::sync::mpsc::Sender<u64>,
    ) -> anyhow::Result<()> {
        let mut path_buf = PathBuf::from(path.clone());
        path_buf.push(heck::AsSnakeCase(data.from_site.clone()).to_string());
        path_buf.push(heck::AsSnakeCase(data.from_ambient.clone()).to_string());
        path_buf.push(heck::AsSnakeCase(data.from_thread.clone()).to_string());
        path_buf.push(data.get_name());
        log::info!(
            "trying to download {} into {:?}",
            data.url,
            path_buf.to_str()
        );

        let mut bytes = self.get_bytes(data.url.to_string()).await?;

        while ArielLoginPage::is_login_page_raw(&bytes) {
            self.login().await?;
            bytes = self.get_bytes(data.url.to_string()).await?;
        }

        let len = bytes.len();

        chunk_done_size_chan.send(len.try_into().unwrap())?;

        if !path_buf.exists() {
            log::trace!("creating path {:?}", path_buf);
            std::fs::create_dir_all(path_buf.parent().unwrap())?
        }

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_buf)?;

        std::io::Write::write_all(&mut file, &bytes)?;

        drop(chunk_done_size_chan);
        Ok(())
    }
}
