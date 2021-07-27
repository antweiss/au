use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method};
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use log::{info};
use env_logger::Env;
//use serde::{Serialize, Deserialize};
//use futures::TryStreamExt; // 0.3.7



async fn echo(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Got {:?}", _req);
    let (parts, body) = _req.into_parts();
    //convert body to a vec
    let body_vec = body::to_bytes(body).await.unwrap().to_vec();

    Ok(Response::new(format!("{{\"method\" : \"{}\",\n\
                            \"uri\" : \"{}\",\n\
                            \"headers\" : {:?},\n\
                            \"version\" : \"{:?}\",\n\
                            \"body\" : {}}}",
                    parts.method,
                    parts.uri,
                    parts.headers,
                    parts.version,
                    if !body_vec.is_empty() { String::from_utf8(body_vec).unwrap()} 
                    else { "\"none\"".to_string() } )
                        .into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let port = 3100;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

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
