use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct SupabaseAuth {
    url: String,
    anon_key: String,
    #[allow(dead_code)]
    service_key: String,  // Reserved for future admin operations
    client: Client,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

#[derive(Clone)]
struct JwksCache {
    keys: Vec<JwkKey>,
    fetched_at: std::time::Instant,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwkKey {
    pub kty: String,
    pub kid: Option<String>,
    pub alg: Option<String>,
    pub n: Option<String>,   // RSA modulus
    pub e: Option<String>,   // RSA exponent
    pub crv: Option<String>, // EC curve
    pub x: Option<String>,   // EC x coordinate
    pub y: Option<String>,   // EC y coordinate
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
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
    pub aud: Option<String>,
    pub iss: Option<String>,
    pub role: Option<String>,
}

impl SupabaseAuth {
    pub fn new(url: String, anon_key: String, service_key: String) -> Self {
        Self {
            url,
            anon_key,
            service_key,
            client: Client::new(),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Fetch JWKS from Supabase
    async fn fetch_jwks(&self) -> Result<Vec<JwkKey>> {
        let url = format!("{}/auth/v1/.well-known/jwks.json", self.url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch JWKS: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("JWKS fetch error {}: {}", status, text));
        }

        let jwks: JwksResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse JWKS: {}", e))?;

        Ok(jwks.keys)
    }

    /// Get JWKS with caching (cache for 5 minutes)
    async fn get_jwks(&self) -> Result<Vec<JwkKey>> {
        const CACHE_DURATION: std::time::Duration = std::time::Duration::from_secs(300);

        // Check cache
        {
            let cache = self.jwks_cache.read().await;
            if let Some(ref cached) = *cache {
                if cached.fetched_at.elapsed() < CACHE_DURATION {
                    return Ok(cached.keys.clone());
                }
            }
        }

        // Fetch fresh JWKS
        let keys = self.fetch_jwks().await?;

        // Update cache
        {
            let mut cache = self.jwks_cache.write().await;
            *cache = Some(JwksCache {
                keys: keys.clone(),
                fetched_at: std::time::Instant::now(),
            });
        }

        Ok(keys)
    }

    /// Find the correct key for a token
    fn find_key_for_token<'a>(&self, keys: &'a [JwkKey], kid: Option<&str>) -> Option<&'a JwkKey> {
        // If token has a kid, find matching key
        if let Some(kid) = kid {
            keys.iter().find(|k| k.kid.as_deref() == Some(kid))
        } else {
            // Otherwise return first key
            keys.first()
        }
    }

    /// Create decoding key from JWK
    fn create_decoding_key(&self, jwk: &JwkKey) -> Result<(DecodingKey, Algorithm)> {
        let alg = jwk.alg.as_deref().unwrap_or("RS256");

        match alg {
            "RS256" | "RS384" | "RS512" => {
                let n = jwk.n.as_ref().ok_or_else(|| anyhow!("Missing RSA modulus"))?;
                let e = jwk.e.as_ref().ok_or_else(|| anyhow!("Missing RSA exponent"))?;
                let key = DecodingKey::from_rsa_components(n, e)
                    .map_err(|e| anyhow!("Invalid RSA key: {}", e))?;
                let algorithm = match alg {
                    "RS384" => Algorithm::RS384,
                    "RS512" => Algorithm::RS512,
                    _ => Algorithm::RS256,
                };
                Ok((key, algorithm))
            }
            "ES256" | "ES384" => {
                let x = jwk.x.as_ref().ok_or_else(|| anyhow!("Missing EC x coordinate"))?;
                let y = jwk.y.as_ref().ok_or_else(|| anyhow!("Missing EC y coordinate"))?;
                let key = DecodingKey::from_ec_components(x, y)
                    .map_err(|e| anyhow!("Invalid EC key: {}", e))?;
                let algorithm = if alg == "ES384" { Algorithm::ES384 } else { Algorithm::ES256 };
                Ok((key, algorithm))
            }
            "HS256" | "HS384" | "HS512" => {
                // For HS256, we need the shared secret which is no longer available
                // Fall back to API verification
                Err(anyhow!("HS256 keys require API verification"))
            }
            _ => Err(anyhow!("Unsupported algorithm: {}", alg)),
        }
    }

    /// Verify JWT token using JWKS
    pub async fn verify_token(&self, token: &str) -> Result<JwtClaims> {
        // Try JWKS verification first
        match self.verify_token_with_jwks(token).await {
            Ok(claims) => return Ok(claims),
            Err(e) => {
                tracing::debug!("JWKS verification failed: {}, falling back to API verification", e);
            }
        }

        // Fall back to API verification (for HS256 or if JWKS fails)
        self.verify_token_with_api(token).await
    }

    /// Verify token using JWKS
    async fn verify_token_with_jwks(&self, token: &str) -> Result<JwtClaims> {
        // Decode header to get kid
        let header = decode_header(token)
            .map_err(|e| anyhow!("Failed to decode JWT header: {}", e))?;

        // Get JWKS
        let keys = self.get_jwks().await?;

        // Find the right key
        let jwk = self
            .find_key_for_token(&keys, header.kid.as_deref())
            .ok_or_else(|| anyhow!("No matching key found in JWKS"))?;

        // Create decoding key
        let (decoding_key, algorithm) = self.create_decoding_key(jwk)?;

        // Set up validation
        let mut validation = Validation::new(algorithm);
        validation.validate_exp = true;
        validation.set_issuer(&[format!("{}/auth/v1", self.url)]);
        validation.set_audience(&["authenticated"]);

        // Decode and verify
        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
            .map_err(|e| anyhow!("JWT verification failed: {}", e))?;

        Ok(token_data.claims)
    }

    /// Verify token by calling Supabase API
    async fn verify_token_with_api(&self, token: &str) -> Result<JwtClaims> {
        let user = self.get_user_from_token(token).await?;

        // Decode token to get claims (without verification since API already verified)
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid JWT format"));
        }

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let decoded = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| anyhow!("Failed to decode JWT payload: {}", e))?;

        let mut claims: JwtClaims = serde_json::from_slice(&decoded)
            .map_err(|e| anyhow!("Failed to parse JWT claims: {}", e))?;

        // Ensure sub matches user ID from API
        claims.sub = user.id.to_string();
        claims.email = Some(user.email);

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

    /// Delete user from Supabase Auth (admin operation)
    /// This permanently removes the user's authentication record
    pub async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        let url = format!("{}/auth/v1/admin/users/{}", self.url, user_id);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .header("apikey", &self.service_key)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete user from Supabase: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase delete user error {}: {}", status, text));
        }

        Ok(())
    }
}
