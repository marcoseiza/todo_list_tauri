use futures::channel::oneshot::Canceled;
use oauth2::{AuthorizationCode, CsrfToken};
use serde::{Deserialize, Serialize};
use std::net::AddrParseError;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: CsrfToken,
}

#[derive(thiserror::Error, Debug)]
pub enum ListenForCodeError {
    #[error(transparent)]
    AddrParse(#[from] AddrParseError),
    #[error(transparent)]
    Canceled(#[from] Canceled),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}

impl Serialize for ListenForCodeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Listen for a code at the specified port.
pub async fn listen_for_code(port: u32) -> anyhow::Result<ReceivedCode, ListenForCodeError> {
    let bind = format!("127.0.0.1:{}", port);
    println!("Listening on: http://{}", bind);
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind(bind).await?;
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await?;

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "You're all logged in! You're free to close this window and go back to the todo_list app.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await?;

            // The server will terminate itself after collecting the first code.
            break Ok(ReceivedCode { code, state });
        }
    }
}
