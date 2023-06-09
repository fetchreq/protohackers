use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};
use smoke_test::{telemetry::{get_subscriber, init_subscriber}, configuration::get_config};
use tracing::{event, Level};

#[tracing::instrument(
    name = "Echo"
)]
async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(req.into_body()))
}

 
#[tokio::main]
async fn main() {

    let subscriber = get_subscriber("echo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    

    let config = get_config().expect("Failed to read configuration");
    let addr = format!("{}:{}",config.application.host, config.application.port);
    let socket_addr: SocketAddr = addr.parse::<SocketAddr>().unwrap();


    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&socket_addr).serve(make_svc);
    event!(Level::INFO, "Starting server on {}", &addr);
    if let Err(e) = server.await {
        eprintln!("server error: {e}")
    } 
}

