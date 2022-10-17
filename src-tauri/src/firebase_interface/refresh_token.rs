use serde::{Deserialize, Serialize};
use std::env;

pub fn get_post_url() -> String {
    format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        env::var("FIREBASE_WEB_API_KEY").unwrap()
    )
}

#[derive(Serialize)]
pub struct RequestBody {
    grant_type: String,
    refresh_token: String,
}

impl RequestBody {
    pub fn new(refresh_token: String) -> Self {
        Self {
            grant_type: "refresh_token".into(),
            refresh_token,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseBody {
    pub refresh_token: String,
    pub id_token: String,
    pub expires_in: String,
    pub project_id: String,
    pub token_type: String,
    pub user_id: String,
}
