use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

// ── GitHub OAuth endpoints ──────────────────────────────────────────────────

const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// Token returned after a successful GitHub OAuth exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubToken {
    /// The OAuth access token.
    pub access_token: String,
    /// Token type (typically "bearer").
    pub token_type: String,
    /// OAuth scopes granted (space-separated).
    pub scope: Option<String>,
    /// Basic user information retrieved after login.
    pub user_info: Option<GitHubUserInfo>,
}

/// Minimal GitHub user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUserInfo {
    pub login: String,
    pub id: u64,
    pub avatar_url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

/// GitHub OAuth flow manager.
///
/// Handles the full OAuth 2.0 authorization-code + PKCE flow for GitHub.
///
/// # Example
///
/// ```no_run
/// # use crate::ui::login::GitHubOAuth;
/// let oauth = GitHubOAuth::new("my-client-id", "my-client-secret");
/// let (auth_url, _csrf, _verifier) = oauth.generate_auth_url();
/// // User visits auth_url in a browser, GitHub redirects with ?code=...
/// // Then call oauth.exchange_code("the-code", verifier).await
/// ```
pub struct GitHubOAuth {
    client: BasicClient,
    http_client: reqwest::Client,
}

impl GitHubOAuth {
    /// Create a new GitHub OAuth handler.
    ///
    /// `client_id` and `client_secret` come from your GitHub OAuth App registration.
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        let client_id = ClientId::new(client_id.into());
        let client_secret = ClientSecret::new(client_secret.into());

        let auth_url = AuthUrl::new(GITHUB_AUTH_URL.to_string())
            .expect("invalid GitHub auth URL");
        let token_url = TokenUrl::new(GITHUB_TOKEN_URL.to_string())
            .expect("invalid GitHub token URL");

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(
                RedirectUrl::new("http://localhost:0/callback".to_string())
                    .expect("invalid redirect URI"),
            );

        let http_client = reqwest::Client::builder()
            .user_agent("HermesOperitApp/0.1")
            .build()
            .expect("failed to create reqwest client");

        Self {
            client,
            http_client,
        }
    }

    /// Generate the authorization URL the user should visit.
    ///
    /// Returns `(url, csrf_token, pkce_verifier)`. The caller must:
    /// 1. Store the `pkce_verifier` (or pass it to `exchange_code` later).
    /// 2. Optionally validate that the returned state matches `csrf_token`.
    pub fn generate_auth_url(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .add_scope(Scope::new("user:email".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), csrf_token, pkce_verifier)
    }

    /// Exchange an authorization code for an access token.
    ///
    /// After the user authorizes in the browser, GitHub redirects with `?code=...&state=...`.
    /// Pass that code and the PKCE verifier from `generate_auth_url` to this method.
    pub async fn exchange_code(
        &self,
        code: impl Into<String>,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<GitHubToken, anyhow::Error> {
        let code = AuthorizationCode::new(code.into());

        let token_result = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http_client)
            .await?;

        let access_token = token_result.access_token().secret().clone();
        let token_type = token_result.token_type().map(|t| t.to_string()).unwrap_or_else(|| "bearer".to_string());
        let scope = token_result.scopes().map(|scopes| {
            scopes
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        });

        // Fetch user info using the access token.
        let user_info = self.fetch_user_info(&access_token).await.ok();

        Ok(GitHubToken {
            access_token,
            token_type,
            scope,
            user_info,
        })
    }

    /// Fetch the authenticated user's profile from the GitHub API.
    async fn fetch_user_info(&self, token: &str) -> Result<GitHubUserInfo, anyhow::Error> {
        let resp = self
            .http_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "HermesOperitApp/0.1")
            .send()
            .await?;

        #[derive(Deserialize)]
        struct GitHubUser {
            login: String,
            id: u64,
            avatar_url: Option<String>,
            name: Option<String>,
            email: Option<String>,
        }

        let user: GitHubUser = resp.json().await?;

        Ok(GitHubUserInfo {
            login: user.login,
            id: user.id,
            avatar_url: user.avatar_url,
            name: user.name,
            email: user.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth_url() {
        let oauth = GitHubOAuth::new("test-client-id", "test-client-secret");
        let (url, _csrf, _verifier) = oauth.generate_auth_url();
        assert!(url.starts_with("https://github.com/login/oauth/authorize"));
        assert!(url.contains("client_id=test-client-id"));
    }
}
