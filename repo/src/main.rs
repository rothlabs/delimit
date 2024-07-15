use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

use graph::*;

type Io = TokioIo<TcpStream>;

#[tokio::main(flavor = "current_thread")] // #[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    let ace = Ace::new(0);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        println!("tokio spawn");
        tokio::spawn(future(io, ace.clone()));
    }
}

async fn future(io: Io, ace: Ace<i32>) {
    let result = http1::Builder::new()
        .serve_connection(io, service_fn(|req| service(req, ace.clone())))
        .await;
    if let Err(err) = result {
        println!("Error serving connection: {:?}", err);
    }
}

async fn service(
    _: Request<impl Body>,
    ace: Ace<i32>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    ace.write(|load| {
        println!("load: {load}");
        *load += 1;
    });
    Ok(Response::new(Full::new(Bytes::from("repo test"))))
}
