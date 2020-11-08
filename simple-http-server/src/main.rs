use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

const VERSION : &str = "0.0.1";

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().as_str();
    let url = req.uri().path();

    match (method, url) {
        ("GET", "/") => {
            let res = Response::builder()
                .status(200)
                .body(Body::from("Okay\n"))
                .unwrap();
            return Ok(res)
        }
        ("GET", "/version") => {
            let res = Response::builder()
                .status(200)
                .body(Body::from(format!("{}\n", VERSION)))
                .unwrap();
            return Ok(res)
        }
        (_, _) => {
            let res = Response::builder()
                .status(404)
                .body(Body::from("Not Found\n"))
                .unwrap();
            return Ok(res)
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Simple HTTP Server : {}", VERSION);
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}