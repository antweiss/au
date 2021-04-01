use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;



async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("Got {:?}", _req);
    Ok(Response::new(format!("{:?}",_req.into_parts()).into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // We'll bind to 127.0.0.1:3100
    let port = 3100;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    info!("Starting the server on {}", port);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}