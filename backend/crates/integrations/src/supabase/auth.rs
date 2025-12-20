use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct SupabaseAuth {
    url: String,
    anon_key: String,
    service_key: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct SupabaseUser {
    pub id: Uuid,
    pub email: String,
    pub user_metadata: UserMetadata,
    pub app_metadata: AppMetadata,
}

#[derive(Debug, Deserialize)]
pub struct UserMetadata {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: Option<String>,
    pub exp: usize,
    pub iat: usize,
}

impl SupabaseAuth {
    pub fn new(url: String, anon_key: String, service_key: String) -> Self {
        Self {
            url,
            anon_key,
            service_key,
            client: Client::new(),
        }
    }

    /// Verify JWT token and extract claims
    /// In production, you'd verify the signature with Supabase's public key
    /// For now, we'll do basic validation
    pub fn verify_token(&self, token: &str) -> Result<JwtClaims> {
        // Split JWT into parts
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid JWT format"));
        }

        // Decode payload (second part)
        let payload = parts[1];
        let decoded = base64::decode_config(payload, base64::URL_SAFE_NO_PAD)
            .map_err(|e| anyhow!("Failed to decode JWT payload: {}", e))?;

        let claims: JwtClaims = serde_json::from_slice(&decoded)
            .map_err(|e| anyhow!("Failed to parse JWT claims: {}", e))?;

        // Check expiration
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if claims.exp < now {
            return Err(anyhow!("Token expired"));
        }

        Ok(claims)
    }

    /// Get user details from Supabase using JWT token
    pub async fn get_user_from_token(&self, token: &str) -> Result<SupabaseUser> {
        let url = format!("{}/auth/v1/user", self.url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("apikey", &self.anon_key)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get user from Supabase: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase API error {}: {}", status, text));
        }

        let user: SupabaseUser = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse user response: {}", e))?;

        Ok(user)
    }

    /// Sign out user (revoke refresh token)
    pub async fn sign_out(&self, token: &str) -> Result<()> {
        let url = format!("{}/auth/v1/logout", self.url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("apikey", &self.anon_key)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to sign out from Supabase: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase sign out error {}: {}", status, text));
        }

        Ok(())
    }
}

// Add base64 dependency to Cargo.toml
