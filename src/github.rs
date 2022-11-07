use psp_types::lsp_types::Url;
use serde::{Serialize, Deserialize};

#[cfg(feature = "http")]
pub mod client {
    use anyhow::Result;
    use serde_json;

    use crate::http::Http;

    use super::Release;

    pub fn latest_release(user: String, repo: String) -> Result<Release> {
        let mut response = Http::get(&format!(
            "https://api.github.com/repos/{user}/{repo}/release/latest"
        ))?;
        let body = response.body_read_all()?;
        let release: Release = serde_json::from_slice(&body)?;
        Ok(release)
    }
}

// use psp_types::lsp_types::Url;
// use serde::{Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    url: Url,
    assets_url: Url,
    upload_url: Url,
    html_url: Url,
    id: u64,
    author: User,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: Url,
    zipball_url: Url,
    body: String,
    reactions: Reactions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    url: Url,
    id: usize,
    node_id: String,
    name: String,
    label: Option<String>,
    uploader: User,
    content_type: String,
    state: String,
    size: u128,
    download_count: u128,
    created_at: String,
    updated_at: String,
    browser_download_url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    login: String,
    id: usize,
    node_id: String,
    avatar_url: Url,
    gravatar_id: String,
    url: Url,
    html_url: Url,
    followers_url: Url,
    following_url: Url,
    gists_url: Url,
    starred_url: Url,
    subscriptions_url: Url,
    organizations_url: Url,
    repos_url: Url,
    events_url: Url,
    received_events_url: Url,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactions {
    url: Url,
    total_count: usize,
    plus_one: usize,
    minus_one: usize,
    laugh: usize,
    hooray: usize,
    confused: usize,
    heart: usize,
    rocket: usize,
    eyes: usize,
}
