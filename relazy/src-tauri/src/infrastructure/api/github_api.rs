// GitHub API client
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GithubAPI {
    client: Client,
}

impl GithubAPI {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("relazy"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn api_url(&self) -> &str {
        GITHUB_API_URL
    }
}
