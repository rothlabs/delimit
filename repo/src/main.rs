pub use config::STORAGE;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use graph::{self, serial::*, snap::*, write::*, Hub, Import, Leaf, Serial, Tray};

mod atlas;
mod config;
#[cfg(test)]
mod tests;

type Io = TokioIo<TcpStream>;

#[tokio::main] // #[tokio::main(flavor = "current_thread")]
pub async fn main() -> graph::Result<()> {
    // Result<(), Box<dyn std::error::Error + Send + Sync>>
    pretty_env_logger::init();
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    let ace = Leaf::new(Tray::I32(0));
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        println!("tokio spawn");
        tokio::spawn(future(io, ace.clone()));
    }
}

async fn future(io: Io, ace: Leaf) {
    let result = http1::Builder::new()
        .serve_connection(io, service_fn(|req| service(req, ace.clone())))
        .await;
    if let Err(err) = result {
        println!("Error serving connection: {:?}", err);
    }
}

async fn service(_: Request<impl Body>, ace: Leaf) -> Result<Response<Full<Bytes>>, Infallible> {
    ace.write(|tray| {
        if let Tray::I32(value) = tray {
            println!("tray: {value}");
            *value += 1;
        }
    })
    .ok();
    Ok(Response::new(Full::new(Bytes::from("repo test"))))
}
