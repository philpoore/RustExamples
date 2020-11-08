use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Mutex, Arc};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

const VERSION : &str = "0.0.1";

fn response_200() -> Response<Body> {
    Response::builder()
        .status(200)
        .body(Body::from("Okay\n"))
        .unwrap()
}

fn response_404() -> Response<Body> {
    Response::builder()
        .status(404)
        .body(Body::from("Not Found\n"))
        .unwrap()
}

async fn handle_request(req: Request<Body>, cache: Arc<Mutex<Box<HashMap<String, String>>>>) -> Result<Response<Body>, Infallible> {
    let method = req.method().as_str();
    let url = req.uri().path();

    match (method, url) {
        ("GET", "/") => {
            return Ok(response_200())
        }
        ("GET", key) => {
            let key_clone = key.to_string();
            let data = &*cache.lock().unwrap();

            match data.get(&key_clone) {
                None => {
                    Ok(response_404())
                }
                Some(value) => {
                    let res = Response::builder()
                        .status(200)
                        .body(Body::from(format!("{}\n", value)))
                        .unwrap();
                    Ok(res)
                }
            }
        }
        ("POST", key) => {
            let key_clone = key.to_string();
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let mut data = cache.lock().unwrap();
            
            println!("Body = {:?}", body);
            // Insert item into hashmap
            data.insert(key_clone.clone(), String::from_utf8(body.to_vec()).unwrap());

            match data.get(&key_clone) {
                Some(value) => {
                    let res = Response::builder()
                        .status(200)
                        .body(Body::from(format!("{}\n", value)))
                        .unwrap();
                    Ok(res)
                }
                None => {
                    Ok(response_404())
                }
            }
        }
        (_, _) => {
            Ok(response_404())
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hash Server : {}", VERSION);

    let cache : Arc<Mutex<Box<HashMap<String, String>>>> = Arc::new(Mutex::new(Box::new(HashMap::new())));
    
    let make_svc = make_service_fn(move |_conn| {
        let cache_ref = cache.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| {
            handle_request(req, cache_ref.clone())
        })) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}