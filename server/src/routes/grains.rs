extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use self::rustc_serialize::json;

pub fn get_grains(req: &mut Request) -> IronResult<Response> {
    println!("grains: {:?}", req);
    let ref query = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("_");
    let payload = json::encode(query).unwrap();
    Ok(Response::with((status::Ok, payload)))
}
