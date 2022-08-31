use async_trait::async_trait;

use super::{page::ArielTitlePage, ArielUserConfig};

pub mod http;

#[async_trait]
pub trait ArielMiddleware: Sync + Send + std::fmt::Debug {
    fn new(config: ArielUserConfig) -> Self
    where
        Self: Sized;
    async fn login(&mut self) -> anyhow::Result<()>;
    async fn search(&mut self, course_name: &str) -> anyhow::Result<Vec<ArielTitlePage>>;
    async fn get(&mut self, url: String) -> anyhow::Result<(String, String)>;
    async fn post(
        &mut self,
        url: String,
        form: Vec<(String, String)>,
    ) -> anyhow::Result<(String, String)>;
    async fn is_logged_in(&mut self) -> anyhow::Result<()>;
}
