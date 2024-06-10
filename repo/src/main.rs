use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use graph::repo::Repo;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::body::Body;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

type Io = TokioIo<TcpStream>;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    let repo = Repo::new();
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        println!("tokio spawn");
        tokio::spawn(future(io, repo.clone()));
    }
}

async fn future(io: Io, repo: Repo) {
    let result = http1::Builder::new()
        .serve_connection(io, service_fn(|req| service(req, repo.clone()))).await;
    if let Err(err) = result {
        println!("Error serving connection: {:?}", err);
    }
}

async fn service(_: Request<impl Body>, repo: Repo) -> Result<Response<Full<Bytes>>, Infallible> {
    if let Ok(mut count) = repo.0.count.lock() {
        *count += 1;
        println!("count: {count}");
    } else {
        println!("did not lock");
    }
    Ok(Response::new(Full::new(Bytes::from("repo test"))))
}