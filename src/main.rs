extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::io::Read;

pub fn is_prime(x: i64) -> bool {
    if x < 2 { return false; }
    if x == 2 { return true; }
    if x % 2 == 0 { 
        return false;
    } else {
        let limit = (x as f64).sqrt() as i64 + 2;
        let mut i = 3;
        while i < limit {
            if x % i == 0 { return false; }
            i += 2;
        }
   }
   true
}

fn handle_request(request: &mut Request) -> IronResult<Response> {
    println!("Recieving a request");
    let mut input_num_string = "".to_string();
    request.body.read_to_string(&mut input_num_string).unwrap();

    let answer_string: String;
    match input_num_string.parse::<i64>() {
        Ok(n) => answer_string = is_prime(n).to_string(),
        Err(_) => answer_string = "The input is not a valid integer".to_string(),
    }
    
    Ok(Response::with((status::Ok, answer_string)))
}

fn main() {
    let mut router = Router::new();
    router.post("/is_prime", handle_request);
    Iron::new(router).http("0.0.0.0:8088").unwrap();
}
