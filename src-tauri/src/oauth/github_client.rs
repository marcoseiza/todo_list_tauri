use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn make_github_client() -> anyhow::Result<BasicClient> {
    let github_client_id = ClientId::new(dotenv!("GITHUB_CLIENT_ID").to_string());
    let github_client_secret = ClientSecret::new(dotenv!("GITHUB_CLIENT_SECRET").to_string());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    Ok(client)
}
