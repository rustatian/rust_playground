use hyper::{Server, Response, Body, Request, Error, Method, StatusCode};
use hyper::service::{service_fn};
use futures::{Future, future};
use std::sync::{Arc, Mutex};
use slab::Slab;
use std::net::SocketAddr;

// r# is a multiline string in rust, ending with #
const INDEX: &'static str = r#"
<!doctype html>
 <html>
     <head>
         <title>Rust Microservice</title>
     </head>
     <body>
         <h3>Rust Microservice</h3>
     </body>
 </html>
"#;

const USER_PATH: &str = "/user/";


type UserId = u64;


struct UserData;

type UserDb = Arc<Mutex<Slab<UserData>>>;

fn main() {
    // tuple of the ([u8;4], u16)
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    let builder = Server::bind(&addr);

    let user_db: Arc<Mutex<Slab<UserData>>> = Arc::new(Mutex::new(Slab::new()));

    let server = builder.serve(move || {
        let user_db: Arc<Mutex<Slab<UserData>>> = user_db.clone();
        service_fn(move |req: Request<Body>| microservice_handler(req, &user_db))
    });

    let server = server.map_err(drop);

    hyper::rt::run(server);
}


fn microservice_handler(req: Request<Body>, user_db: &UserDb) -> impl Future<Item=Response<Body>, Error=Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        }
        _ => {
            let response = {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/") => {
                        Response::new(INDEX.into())
                    }
                    (method, path) if path.starts_with(USER_PATH) => {
                        let user_id = path.trim_start_matches(USER_PATH)
                            .parse::<UserId>()
                            .ok()
                            .map(|x| x as usize);
                        unimplemented!();
                    }
                    _ => {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                }
            };
//            let response = Response::builder().status(StatusCode::NOT_FOUND)
//                .body(Body::empty())
//                .unwrap();
            future::ok(response)
        }
    }
}


fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

//        service_fn_ok(|_| {
//            Response::new(Body::from("Almost microservice..."))
//        })
//        // Expects the result to be converted into future with the IntoFuture trait




























