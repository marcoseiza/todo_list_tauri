use serde::{Deserialize, Serialize};
use std::env;

pub fn get_post_url() -> String {
    format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key={}",
        env::var("FIREBASE_WEB_API_KEY").unwrap()
    )
}

#[allow(non_snake_case)]
struct PostBody {
    access_token: String,
    providerId: String,
}

impl Serialize for PostBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            format!(
                "access_token={}&providerId={}",
                self.access_token, self.providerId
            )
            .as_str(),
        )
    }
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct RequestBody {
    postBody: PostBody,
    requestUri: String,
    returnIdpCredential: bool,
    returnSecureToken: bool,
}

impl RequestBody {
    pub fn make_body_for_github(access_token: String) -> Self {
        Self {
            postBody: PostBody {
                access_token,
                providerId: "github.com".into(),
            },
            requestUri: "http://localhost".into(),
            returnIdpCredential: true,
            returnSecureToken: true,
        }
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ResponseBody {
    pub federatedId: String,
    pub providerId: String,
    pub localId: String,
    pub emailVerified: bool,
    pub email: String,
    pub oauthIdToken: Option<String>,
    pub oauthAccessToken: Option<String>,
    pub oauthTokenSecret: Option<String>,
    pub rawUserInfo: String,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub fullName: Option<String>,
    pub displayName: String,
    pub photoUrl: String,
    pub idToken: String,
    pub refreshToken: String,
    pub expiresIn: String,
    pub needConfirmation: Option<bool>,
}
