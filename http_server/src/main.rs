use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ready") => {
            *response.status_mut() = StatusCode::OK;
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 34578));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
