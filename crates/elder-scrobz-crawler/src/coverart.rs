use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

use crate::ScrobbleResolver;

impl ScrobbleResolver {
    pub async fn download_image(&self, url: &str, release_mbid: &str) -> anyhow::Result<()> {
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            let path = format!("{release_mbid}.jpg");
            let path = self.coverart_path.join(path);
            let mut file = File::create(&path).await?;
            file.write_all(&bytes).await?;
            info!("Image downloaded successfully {}", path.display());
        } else {
            error!("Failed to download image. Status: {}", response.status());
        }

        Ok(())
    }
}
