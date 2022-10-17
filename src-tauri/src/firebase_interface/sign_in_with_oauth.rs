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

#[derive(Deserialize, Debug, Clone)]
pub struct GithubUserInfo {
    pub avatar_url: String,
    pub html_url: String,
    // Extra GithubUserInfoTypes
    // "avatar_url": String("https://avatars.githubusercontent.com/u/63314030?v=4"),
    // "bio": String("College student looking for professional experience and mentorship from accomplished developers. My interests lie in bridging the gap between the analytical and"),
    // "blog": String(""),
    // "company": Null,
    // "created_at": String("2020-04-07T18:31:16Z"),
    // "email": Null,
    // "events_url": String("https://api.github.com/users/marcoseiza/events{/privacy}"),
    // "followers": Number(3),
    // "followers_url": String("https://api.github.com/users/marcoseiza/followers"),
    // "following": Number(0),
    // "following_url": String("https://api.github.com/users/marcoseiza/following{/other_user}"),
    // "gists_url": String("https://api.github.com/users/marcoseiza/gists{/gist_id}"),
    // "gravatar_id": String(""),
    // "hireable": Null,
    // "html_url": String("https://github.com/marcoseiza"),
    // "id": Number(63314030),
    // "location": Null,
    // "login": String("marcoseiza"),
    // "name": String("Marcos Eizayaga"),
    // "node_id": String("MDQ6VXNlcjYzMzE0MDMw"),
    // "organizations_url": String("https://api.github.com/users/marcoseiza/orgs"),
    // "public_gists": Number(1),
    // "public_repos": Number(18),
    // "received_events_url": String("https://api.github.com/users/marcoseiza/received_events"),
    // "repos_url": String("https://api.github.com/users/marcoseiza/repos"),
    // "site_admin": Bool(false),
    // "starred_url": String("https://api.github.com/users/marcoseiza/starred{/owner}{/repo}"),
    // "subscriptions_url": String("https://api.github.com/users/marcoseiza/subscriptions"),
    // "twitter_username": Null,
    // "type": String("User"),
    // "updated_at": String("2022-10-10T14:05:40Z"),
    // "url": String("https://api.github.com/users/marcoseiza"),
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseBody {
    pub federatedId: String,
    pub providerId: String,
    pub localId: String,
    pub emailVerified: bool,
    pub email: Option<String>,
    pub oauthIdToken: Option<String>,
    pub oauthAccessToken: Option<String>,
    pub oauthTokenSecret: Option<String>,
    #[serde(deserialize_with = "deserialize_github_user_info_string")]
    pub rawUserInfo: GithubUserInfo,
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

fn deserialize_github_user_info_string<'de, D>(deserializer: D) -> Result<GithubUserInfo, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    serde_json::from_str(s.as_str()).map_err(serde::de::Error::custom)
}

pub async fn get_response(access_token: String) -> Result<ResponseBody, reqwest::Error> {
    reqwest::Client::new()
        .post(get_post_url())
        .json(&RequestBody::make_body_for_github(access_token))
        .send()
        .await?
        .json::<ResponseBody>()
        .await
}
