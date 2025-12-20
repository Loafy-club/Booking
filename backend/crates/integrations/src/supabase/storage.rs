use anyhow::{anyhow, Result};
use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SupabaseStorage {
    url: String,
    service_key: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub key: String,
}

impl SupabaseStorage {
    pub fn new(url: String, service_key: String) -> Self {
        Self {
            url,
            service_key,
            client: Client::new(),
        }
    }

    /// Upload file to Supabase Storage
    pub async fn upload_file(
        &self,
        bucket: &str,
        path: &str,
        file_bytes: Vec<u8>,
        content_type: &str,
    ) -> Result<String> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.url, bucket, path
        );

        let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(file_bytes)
                .mime_str(content_type)?);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to upload file to Supabase Storage: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase Storage upload error {}: {}", status, text));
        }

        // Return public URL
        let public_url = format!(
            "{}/storage/v1/object/public/{}/{}",
            self.url, bucket, path
        );

        Ok(public_url)
    }

    /// Delete file from Supabase Storage
    pub async fn delete_file(&self, bucket: &str, path: &str) -> Result<()> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.url, bucket, path
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete file from Supabase Storage: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase Storage delete error {}: {}", status, text));
        }

        Ok(())
    }

    /// Get signed URL for temporary access to private file
    pub async fn get_signed_url(
        &self,
        bucket: &str,
        path: &str,
        expires_in: u64, // seconds
    ) -> Result<String> {
        let url = format!(
            "{}/storage/v1/object/sign/{}/{}",
            self.url, bucket, path
        );

        #[derive(Serialize)]
        struct SignRequest {
            #[serde(rename = "expiresIn")]
            expires_in: u64,
        }

        #[derive(Deserialize)]
        struct SignResponse {
            #[serde(rename = "signedURL")]
            signed_url: String,
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .json(&SignRequest { expires_in })
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get signed URL from Supabase Storage: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase Storage signed URL error {}: {}", status, text));
        }

        let sign_response: SignResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse signed URL response: {}", e))?;

        Ok(format!("{}{}", self.url, sign_response.signed_url))
    }
}
