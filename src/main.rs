use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;
use serde::{Serialize, Deserialize};
use futures::TryStreamExt; // 0.3.7



async fn echo(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Got {:?}", _req);
    let (parts, body) = _req.into_parts();

    Ok(Response::new(format!("{:?}, {:?}",parts, body::to_bytes(body).await).into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let port = 3100;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    info!("Starting the server on {}", port);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}