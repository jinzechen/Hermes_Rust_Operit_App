//! GitHub OAuth 2.0 login flow using the `oauth2` crate with PKCE.

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

/// GitHub OAuth application credentials.
pub struct GitHubOAuth {
    client: BasicClient,
}

/// Result of a successful OAuth exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: Option<String>,
    pub user_info: Option<GitHubUserInfo>,
}

/// GitHub user profile information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUserInfo {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}

impl GitHubOAuth {
    /// Create a new OAuth client with the given GitHub app credentials.
    pub fn new(client_id: &str, client_secret: &str, redirect_url: &str) -> Self {
        let client_id = ClientId::new(client_id.into());
        let client_secret = ClientSecret::new(client_secret.into());
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Invalid GitHub authorize URL");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Invalid GitHub token URL");

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("Invalid redirect URL"));

        Self { client }
    }

    /// Generate the authorization URL that the user must visit in a browser.
    pub fn generate_auth_url(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".into()))
            .add_scope(Scope::new("read:user".into()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), csrf_token, pkce_verifier)
    }

    /// Exchange an authorization code for an access token using a reqwest HTTP client.
    pub fn exchange_code(
        &self,
        code: impl Into<String>,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<GitHubToken, anyhow::Error> {
        let code = AuthorizationCode::new(code.into());
        let http_client = reqwest::blocking::Client::new();

        // Use oauth2's built-in reqwest integration (sync via closure)
        let token_result = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(oauth2::reqwest::http_client)?;

        let access_token = token_result.access_token().secret().clone();
        let token_type = match token_result.token_type() {
            oauth2::basic::BasicTokenType::Bearer => "bearer".to_string(),
            _ => "unknown".to_string(),
        };
        let scope = token_result.scopes().map(|scopes| {
            scopes
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        });

        // Fetch user info using the access token.
        let user_info: Option<GitHubUserInfo> = http_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "HermesOperitApp/0.1.0")
            .header("Accept", "application/vnd.github+json")
            .send()
            .ok()
            .and_then(|r| r.json().ok());

        Ok(GitHubToken {
            access_token,
            token_type,
            scope,
            user_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth_url() {
        let oauth = GitHubOAuth::new(
            "test-client-id",
            "test-client-secret",
            "hermesapp://callback",
        );
        let (url, _csrf, _verifier) = oauth.generate_auth_url();
        assert!(url.contains("github.com/login/oauth/authorize"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("redirect_uri=hermesapp%3A%2F%2Fcallback"));
    }
}
