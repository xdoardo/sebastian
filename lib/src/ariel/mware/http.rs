use async_trait::async_trait;
use reqwest::Client;

use super::ArielMiddleware;
use crate::ariel::{map::ArielSitemap, ArielUserConfig};

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
        let user_agent =
            String::from("Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0");
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

        crate::ariel::page::ArielLoginPage::is_logged_in(text)
    }

    async fn search(
        &mut self,
        course_name: &str,
    ) -> anyhow::Result<Vec<crate::ariel::page::ArielTitlePage>> {
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
}
