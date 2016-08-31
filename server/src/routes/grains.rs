extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use self::rustc_serialize::json;
use services::sqlite::{SqliteDB};
use persistent::Read;
use models::grain::{list_grains};

macro_rules! get_sqlite_connection {
    ($req:expr) => (match $req.get::<Read<SqliteDB>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                println!("Couldn't get a connection to Sqlite!");
                return Ok(Response::with((status::InternalServerError)));
            }
        },
        Err(_) => {
            println!("Couldn't get the Sqlite pool from the request!");
            return Ok(Response::with((status::InternalServerError)));
        }
    })
}

pub fn get_grains(req: &mut Request) -> IronResult<Response> {
    println!("grains: {:?}", req);
    let conn = get_sqlite_connection!(req);
    match list_grains(conn) {
        Ok(grains) => {
            let payload = json::encode(&grains).unwrap();
            Ok(Response::with((status::Ok, payload)))
        },
        Err(e) => {
            println!("Errored: {:?}", e);
            Ok(Response::with((status::InternalServerError)))
        }
    }
    // let ref query = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("_");
    // let payload = json::encode(query).unwrap();
    // Ok(Response::with((status::Ok, payload)))
}
