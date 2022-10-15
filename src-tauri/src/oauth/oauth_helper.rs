use futures::{
    channel::oneshot::{self, Canceled},
    prelude::*,
    task::{Context, Poll},
};
use hyper::{body::Body, server, service, Request, Response};
use oauth2::{AuthorizationCode, CsrfToken};
use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, SocketAddr};
use tower_service::Service;

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
    AddrParseError(#[from] AddrParseError),
    #[error(transparent)]
    Canceled(#[from] Canceled),
}

impl Serialize for ListenForCodeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Interface to the server.
pub struct Server {
    channel: Option<oneshot::Sender<ReceivedCode>>,
}

impl Service<Request<Body>> for Server {
    type Response = Response<Body>;
    type Error = String;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if let Ok(code) =
            serde_urlencoded::from_str::<ReceivedCode>(req.uri().query().unwrap_or(""))
        {
            if let Some(channel) = self.channel.take() {
                let _ = channel.send(code);
            }
        }

        Box::pin(future::ok(Response::new(Body::from(format!("You're all logged in! You're free to close this window and go back to the todo_list app.")))))
    }
}

/// Listen for a code at the specified port.
pub async fn listen_for_code(port: u32) -> anyhow::Result<ReceivedCode, ListenForCodeError> {
    let bind = format!("127.0.0.1:{}", port);
    println!("Listening on: http://{}", bind);

    let addr: SocketAddr = str::parse(&bind)?;

    let (tx, rx) = oneshot::channel::<ReceivedCode>();

    let mut channel = Some(tx);

    let make_svc = service::make_service_fn(|_| {
        let channel = channel.take().expect("channel is not available");
        let mut server = Server {
            channel: Some(channel),
        };
        let service = service::service_fn(move |req| server.call(req));

        async move { Ok::<_, hyper::Error>(service) }
    });

    let server_future = server::Server::bind(&addr).serve(make_svc);

    let mut server_future = server_future.fuse();
    let mut rx = rx.fuse();

    futures::select! {
        _ = server_future => panic!("server exited for some reason"),
        received = rx => Ok(received?),
    }
}
