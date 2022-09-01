use m3u8_rs::Playlist;
use url::Url;

use super::http::HttpArielMiddleware;

impl HttpArielMiddleware {
    #[async_recursion::async_recursion]
    pub(super) async fn get_m3u8_segments(
        &mut self,
        uri: Url,
    ) -> anyhow::Result<Vec<m3u8_rs::MediaSegment>> {
        let bytes = self.get_bytes(uri.to_string()).await?;
        let parsed = m3u8_rs::parse_playlist_res(&bytes);

        match parsed {
            Ok(Playlist::MasterPlaylist(pl)) => {
                if pl.variants.len() == 0 {
                    return Ok(vec![]);
                } else {
                    let variant = &pl.variants[0];
                    return self.get_m3u8_segments(uri.join(&variant.uri)?).await;
                }
            }
            Ok(Playlist::MediaPlaylist(pl)) => {
                let mut segs = pl.segments;
                for seg in &mut segs {
                    seg.uri = uri.join(&seg.uri).unwrap().to_string();
                }
                Ok(segs)
            }
            Err(e) => anyhow::bail!("{}", e),
        }
    }
}
