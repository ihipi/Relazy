use octocrab::models::Author;
use octocrab::Octocrab;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GithubAPI {
    octo: Octocrab,
}

impl GithubAPI {
    pub fn new(token: &str) -> Self {
        let octo = Octocrab::builder()
            .personal_token(token.to_string())
            .base_uri(GITHUB_API_URL)
            .unwrap()
            .build()
            .expect("Failed to create Octocrab client");

        Self { octo }
    }

    pub fn octo(&self) -> &Octocrab {
        &self.octo
    }

    pub fn api_url(&self) -> &str {
        GITHUB_API_URL
    }

    pub async fn get_authenticated_user(&self) -> Result<Author, octocrab::Error> {
        let user = self.octo.current().user().await?;
        Ok(user)
    }
}
