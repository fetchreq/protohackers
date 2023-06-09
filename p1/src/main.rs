use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}};
use p1::telemetry::{get_subscriber, init_subscriber};
use tracing::{event, Level};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonRequest {
    method: String,
    number: serde_json::Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse {
    method: &'static str,
    prime: bool
} 

impl JsonResponse {
    fn new(x: u64) -> Self {
        Self { method: "isPrime", prime: is_prime(x)}
    }

    fn default() -> Self {
        Self { method: "isPrime", prime: false}
    }

    fn malformed() -> Self {
        Self { method: "Malformed", prime: is_prime(1)}
    }
}



#[tracing::instrument(
    name = "json"
)]
async fn hello_world(req: Request<Body>) -> Result<Response<String>, Infallible> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    
    let resp = match check_body(&body) { 
        Ok(resp) => serde_json::to_string(&resp).unwrap(),
        Err(_e) => { 
            serde_json::to_string(&JsonResponse::malformed()).unwrap()
        }
    };
    Ok(Response::new(resp))
}

fn check_body(body: &str) -> anyhow::Result<JsonResponse> {
    if let Ok(body) = serde_json::from_str::<JsonRequest>(body) {
        println!("{:?}", body);
        return match body.method.as_str() {
            "isPrime" if !body.number.is_f64()=> { 
                return Ok(JsonResponse::new(body.number.as_u64().unwrap()));
            }
            "isPrime" if body.number.is_f64()=> { 
                return Ok(JsonResponse::default())
            }
            _ => Err(anyhow::anyhow!("Wrong method")) 
        }
    }
    Err(anyhow::anyhow!("Parse Error"))
    
}

 
#[tokio::main]
async fn main() {

    let subscriber = get_subscriber("echo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    

    let addr: SocketAddr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);
    event!(Level::INFO, "Starting server");
    if let Err(e) = server.await {
        eprintln!("server error: {e}")
    } 
}

fn find_factor(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    } 

    for n in (1..).map(|m|  2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        }
    }

    x
}

fn is_prime(x: u64) -> bool {
    if x <= 1 {
        return false;
    }
    find_factor(x) == x
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn body_with_prime() -> Result<(), &'static str> {
        let body = "{\"method\":\"isPrime\",\"number\":23}";
        match check_body(body) {
            Ok(resp) => {
                assert_eq!(resp.method, "isPrime");
                assert_eq!(resp.prime, true);
                Ok(())
            }
            Err(_)=> Err("should be some")
        }
    }
    
    #[test]
    fn body_with_wrong_method() -> Result<(), &'static str> {
        let body = "{\"method\":\"isNumber\",\"number\":23}";
        match check_body(body) {
            Ok(_) => Err("Should be error"),
            Err(_) => { 
                Ok(()) 
            }
        }
    }

    #[test]
    fn body_with_wrong_number_params() -> Result<(), &'static str> {
        let body = "{\"method\":\"isPrime\",\"number\":\"not a number\"}";
        match check_body(body) {
            Ok(_) => Err("Should be error"),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn body_with_non_prime() -> Result<(), &'static str> {
        let body = "{\"method\":\"isPrime\",\"number\":25}";
        match check_body(body) {
            Ok(resp) => {
                assert_eq!(resp.method, "isPrime");
                assert_eq!(resp.prime, false);
                Ok(())
            }
            Err(_) => Err("should be ok")
        }
    }


    #[test]
    fn body_with_floating_point() -> Result<(), &'static str> {
        let body = "{\"method\":\"isPrime\",\"number\":23.1}";
        match check_body(body) {
            Ok(resp) => {
                assert_eq!(resp.prime, false);
                Ok(())
            }
            Err(_) => Err("should be Ok")
        }

    }

    #[test]
    fn body_with_extra() -> Result<(), &'static str> {
        let body = "{\"method\":\"isPrime\",\"number\":23,\"something\":\"value\"}";
        match check_body(body) {
            Ok(_) => Ok(()),
            Err(_) => Err("should be some")
        }

    }

    #[test]
    fn check_is_prime() {
        assert_eq!(is_prime(23), true)
    }

    #[test]
    fn check_is_prime_false() {
        assert_eq!(is_prime(25), false)
    }
}



