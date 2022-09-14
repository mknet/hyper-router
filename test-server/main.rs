extern crate hyper;
extern crate hyper_router;
extern crate tokio;

use std::convert::Infallible;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

use hyper::{Body, Method, Request, Response, Server, server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper_router::{Route, RouterBuilder, RouterService};

fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello World";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::PATCH, "/world").using(request_handler))
        .build();

    Ok(RouterService::new(router))
}


#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let make_service = make_service_fn(move |conn: &AddrStream| {

        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
                let router = RouterBuilder::new()
                    .add(Route::get("/hello").using(request_handler))
                    .add(Route::from(Method::PATCH, "/world").using(request_handler))
                    .build();

                Ok::<_, Infallible>(
                    match router.find_handler(&req) {
                        Ok(handler) => handler(req),
                        Err(status_code) => Response::new(Body::from(format!("Hello, {}!", req.uri())))
                    }
                )
            }))
        }
    });


    let server = Server::bind(&addr)
        .serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
