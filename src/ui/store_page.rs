use serde::{Deserialize, Serialize};

/// GitHub release API base URL for the Operit package marketplace.
const MARKET_API_BASE: &str =
    "https://api.github.com/repos/OperitScriptMarket/OperitPackageMarket/releases";

/// A plugin listing entry displayed in the store browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginListing {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub downloads: u64,
    pub category: String,
}

/// GitHub release asset from the API.
#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    #[serde(rename = "browser_download_url")]
    download_url: String,
    #[serde(rename = "download_count")]
    download_count: u64,
}

/// GitHub release from the API.
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    #[serde(rename = "tag_name")]
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    assets: Vec<GitHubAsset>,
    #[serde(default)]
    author: Option<GitHubAuthor>,
}

#[derive(Debug, Deserialize)]
struct GitHubAuthor {
    login: String,
}

/// Browser for the Operit plugin store.
///
/// Fetches listings from GitHub Releases and supports searching/filtering.
pub struct StoreBrowser {
    client: reqwest::Client,
    cached_listings: Vec<PluginListing>,
}

impl StoreBrowser {
    /// Create a new `StoreBrowser` with a default HTTP client.
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent("HermesOperitApp/0.1")
            .build()?;
        Ok(Self {
            client,
            cached_listings: Vec::new(),
        })
    }

    /// Fetch all plugin listings from the remote marketplace.
    pub async fn fetch_listings(&mut self) -> Result<Vec<PluginListing>, anyhow::Error> {
        let resp = self
            .client
            .get(MARKET_API_BASE)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        let releases: Vec<GitHubRelease> = resp.json().await?;

        let mut listings = Vec::new();
        for release in releases {
            let author = release
                .author
                .as_ref()
                .map(|a| a.login.clone())
                .unwrap_or_else(|| "unknown".to_string());

            let total_downloads: u64 = release.assets.iter().map(|a| a.download_count).sum();

            // Parse category from release body (convention: `category: <name>`)
            let category = release
                .body
                .as_deref()
                .and_then(|body| {
                    body.lines()
                        .find(|line| line.trim().starts_with("category:"))
                        .map(|line| {
                            line.trim()
                                .trim_start_matches("category:")
                                .trim()
                                .to_string()
                        })
                })
                .unwrap_or_else(|| "general".to_string());

            let plugin_name = release
                .name
                .clone()
                .unwrap_or_else(|| release.tag_name.clone());

            listings.push(PluginListing {
                name: plugin_name,
                description: release.body.unwrap_or_default(),
                version: release.tag_name,
                author,
                downloads: total_downloads,
                category,
            });
        }

        self.cached_listings = listings.clone();
        Ok(listings)
    }

    /// Search cached listings by a query string (case-insensitive match on name + description).
    pub fn search(&self, query: &str) -> Vec<&PluginListing> {
        let q = query.to_lowercase();
        self.cached_listings
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
            })
            .collect()
    }

    /// Get details for a plugin by exact name (from cache).
    pub fn get_details(&self, name: &str) -> Option<&PluginListing> {
        self.cached_listings
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }

    /// Return a reference to all cached listings.
    pub fn cached(&self) -> &[PluginListing] {
        &self.cached_listings
    }
}

impl Default for StoreBrowser {
    fn default() -> Self {
        Self::new().expect("failed to create StoreBrowser HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_empty_cache() {
        let browser = StoreBrowser::new().unwrap();
        let results = browser.search("test");
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_details_missing() {
        let browser = StoreBrowser::new().unwrap();
        assert!(browser.get_details("nonexistent").is_none());
    }
}
