extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

mod routes;
mod services;
use std::fs;
use std::path::{PathBuf, Path};
use iron::prelude::*;
use iron::typemap::Key;
use router::Router;
use staticfile::Static;
use mount::Mount;
use routes::grains::{get_grains};
// use services::sqlite::sqlite;

fn main() {
    let app_path_buf = PathBuf::from("./../app/src");
    let app_path = fs::canonicalize(&app_path_buf).unwrap();

    let mut router = Router::new();
    router.get("/grains/:id", move |r: &mut Request| get_grains(r));

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(app_path.to_str().unwrap())));
    mount.mount("/api", router);

    let mut chain = Chain::new(mount);
    //chain.link(Write::<sqlite>::both(DatabaseConnection::in_memory()));

    Iron::new(chain).http("localhost:3000").unwrap();
}
