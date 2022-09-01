use async_trait::async_trait;

use super::{
    page::{ArielPageData, ArielTitlePage},
    ArielUserConfig,
};

pub mod http;
mod m3u8;
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

    async fn download<'a>(
        &mut self,
        path: String,
        data: ArielPageData,
        chunk_done_size_chan: std::sync::mpsc::Sender<u64>,
    ) -> anyhow::Result<()>;

    async fn get_size<'a>(&mut self, data: &'a ArielPageData) -> anyhow::Result<u64>;
}
