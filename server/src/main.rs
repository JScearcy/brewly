extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

fn main() {

    let mut router = Router::new();
    router.get("/", move |r: &mut Request| hello_world(r, "Hello World!"));
    router.get("/:echo", move |r: &mut Request| echo(r));

    Iron::new(router).http("localhost:3000").unwrap();
}

fn hello_world(req: &mut Request, msg: &'static str) -> IronResult<Response> {
    //let payload = json::encode(msg).unwrap();
    println!("Main: {:?}", req);
    Ok(Response::with((status::Ok, msg)))
}

fn echo(req: &mut Request) -> IronResult<Response> {
    println!("Echo: {:?}", req);
    Ok(Response::with((status::Ok)))
}
