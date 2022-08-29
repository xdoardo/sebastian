use super::ArielMiddleware;
use crate::ariel::{map::ArielSitemap, ArielUserConfig};

#[derive(Debug)]
pub struct HttpArielMiddleware {
    config: ArielUserConfig,
    sitemap: ArielSitemap,
    cookies: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
    user_agent: String,
}

impl ArielMiddleware for HttpArielMiddleware {
    fn new(config: ArielUserConfig) -> Self
    where
        Self: Sized,
    {
        HttpArielMiddleware {
            config,
            sitemap: ArielSitemap::default(),
            cookies: std::sync::Arc::new(reqwest_cookie_store::CookieStoreMutex::default()),
            user_agent: String::from(
                "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0",
            ),
        }
    }

    fn login(&mut self) -> anyhow::Result<()> {
        let (_, text) = self.post(
            self.sitemap.login_url.clone(),
            vec![
                ("hdnSilent".into(), "true".into()),
                ("tbLogin".into(), self.config.username.clone()),
                ("tbPassword".into(), self.config.password.clone()),
            ],
            true,
            true,
        )?;

        crate::ariel::page::ArielLoginPage::is_logged_in(text)
    }

    fn search(
        &mut self,
        course_name: &str,
    ) -> anyhow::Result<Vec<crate::ariel::page::ArielTitlePage>> {
        let res = self.post(
            self.sitemap.search_url.clone(),
            vec![("keyword".into(), course_name.into())],
            true,
            true,
        )?;
        Ok(crate::ariel::page::ArielSearchPage::title_pages(res.1))
    }

    fn get(&mut self, url: String, redirect: bool) -> anyhow::Result<(String, String)> {
        self.get(url, redirect, true)
    }

    fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
        redirect: bool,
    ) -> anyhow::Result<(String, String)> {
        self.post(url, form, redirect, true)
    }

    fn is_logged_in(&mut self) -> anyhow::Result<()> {
        let (_, page) = self.get(self.sitemap.home_page_url.clone(), true, true)?;
        crate::ariel::page::ArielLoginPage::is_logged_in(page)
    }
}

impl HttpArielMiddleware {
    fn get(
        &mut self,
        url: String,
        redirect: bool,
        update_cookies: bool,
    ) -> anyhow::Result<(String, String)> {
        let mut builder = reqwest::blocking::Client::builder();

        if update_cookies {
            builder = builder.cookie_store(true);
        }

        builder = builder
            .cookie_provider(self.cookies.clone())
            .user_agent(self.user_agent.clone());

        if redirect {
            builder = builder.redirect(reqwest::redirect::Policy::limited(15));
        } else {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        let client = builder.build()?;

        let res = client.get(url.clone()).send()?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("posting to url '{}', status {}", url, status)
        }

        Ok((res.url().to_string(), res.text()?))
    }

    fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
        redirect: bool,
        update_cookies: bool,
    ) -> anyhow::Result<(String, String)> {
        let mut builder = reqwest::blocking::Client::builder();

        if update_cookies {
            builder = builder.cookie_store(true);
        }

        builder = builder
            .cookie_provider(self.cookies.clone())
            .user_agent(self.user_agent.clone());

        log::info!("{:?}", self.cookies);

        if redirect {
            builder = builder.redirect(reqwest::redirect::Policy::limited(15));
        } else {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        let client = builder.build()?;

        let req = client.post(url.clone()).form(&form);
        log::info!("{:?}", req);
        let res = req.send()?;
        let status = res.status().clone();

        if !status.is_success() {
            anyhow::bail!("posting to url '{}', status {}", url, status)
        }

        Ok((res.url().to_string(), res.text()?))
    }
}
