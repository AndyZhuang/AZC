//! Storage abstraction for package files.

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use anyhow::Result;

/// Storage backend for package files
pub struct Storage {
    client: Client,
    bucket: String,
}

impl Storage {
    /// Create a new storage client
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        
        let bucket = std::env::var("S3_BUCKET")
            .unwrap_or_else(|_| "azc-packages".to_string());

        Ok(Self { client, bucket })
    }

    /// Upload a package file
    pub async fn upload(&self, name: &str, version: &str, data: &[u8]) -> Result<String> {
        let key = format!("{}/{}.azc", name, version);
        
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(data.to_vec().into())
            .send()
            .await?;

        Ok(key)
    }

    /// Download a package file
    pub async fn download(&self, name: &str, version: &str) -> Result<Vec<u8>> {
        let key = format!("{}/{}.azc", name, version);
        
        let output = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;

        let data = output.body.collect().await?.to_vec();
        Ok(data)
    }

    /// Delete a package file
    pub async fn delete(&self, name: &str, version: &str) -> Result<()> {
        let key = format!("{}/{}.azc", name, version);
        
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;

        Ok(())
    }
}