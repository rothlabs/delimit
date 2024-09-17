use std::net::SocketAddr;
use std::path::Path;

use bytes::Bytes;
use futures_util::TryStreamExt;
use http_body_util::{combinators::BoxBody, BodyExt, Full, StreamBody};
use hyper::body::{Frame, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;

use config::{CLIENT, STATIC};
use index::index;

mod config;
mod index;

const BOOT: &str = "/boot.js";
pub const INIT: &str = "/client.js";
const MAIN: &str = "/client_bg.wasm";

static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // This address is localhost
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        //
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (tcp, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tcp);

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(|req| handle(req)))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

type RequestResult = hyper::Result<Response<BoxBody<Bytes, std::io::Error>>>;

async fn handle(req: Request<Incoming>) -> RequestResult {
    println!("request");
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => text(index().await.unwrap()),
        (&Method::GET, BOOT) => static_file(BOOT).await,
        (&Method::GET, INIT) => client_file(INIT).await,
        (&Method::GET, MAIN) => client_file(MAIN).await,
        _ => not_found(),
    }
}

fn text<T: Into<Bytes>>(chunk: T) -> RequestResult {
    Ok(Response::new(
        Full::new(chunk.into()).map_err(|e| match e {}).boxed(),
    ))
}

fn not_found() -> RequestResult {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOTFOUND.into()).map_err(|e| match e {}).boxed())
        .unwrap())
}

async fn static_file(path: &str) -> RequestResult {
    send_file(STATIC.to_owned() + path).await
}

async fn client_file(path: &str) -> RequestResult {
    send_file(CLIENT.to_owned() + path).await
}

async fn send_file(path: String) -> RequestResult {
    let file = File::open(&path).await;
    if file.is_err() {
        eprintln!("ERROR: Unable to open file.");
        return not_found();
    }
    let file: File = file.unwrap();
    let reader_stream = ReaderStream::new(file);
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();
    let content_type = match Path::new(&path).extension().unwrap().to_str().unwrap() {
        "js" => "text/javascript",
        "wasm" => "application/wasm",
        _ => "text/plain",
    };
    let response = Response::builder()
        .header("Content-Type", content_type)
        .status(StatusCode::OK)
        .body(boxed_body)
        .unwrap();
    Ok(response)
}

// struct Paths<'a> {
//     pub app: &'a str,
// }

// const PATH: Paths<'static> = Paths {
//     app: "app.js",
// };

// // We create some utility functions to make Empty and Full bodies
// // fit our broadened Response body type.
// fn empty() -> BoxBody<Bytes, hyper::Error> {
//     Empty::<Bytes>::new()
//         .map_err(|never| match never {})
//         .boxed()
// }
