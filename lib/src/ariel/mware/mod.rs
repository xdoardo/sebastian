use super::{page::ArielTitlePage, ArielUserConfig};

pub mod http;

pub trait ArielMiddleware: Sync + Send + std::fmt::Debug {
    fn new(config: ArielUserConfig) -> Self
    where
        Self: Sized;
    fn login(&mut self) -> anyhow::Result<()>;
    fn search(&mut self, course_name: &str) -> anyhow::Result<Vec<ArielTitlePage>>;
    fn get(&mut self, url: String, redirect: bool) -> anyhow::Result<(String, String)>;
    fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
        redirect: bool,
    ) -> anyhow::Result<(String, String)>;
    fn is_logged_in(&mut self) -> anyhow::Result<()>;
}
