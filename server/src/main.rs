extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate persistent;

mod services;
mod routes;
mod models;

use std::{fs};
use std::path::{PathBuf, Path};
use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use routes::grains::{get_grains};
use services::sqlite::{SqliteDB};
use r2d2_sqlite::SqliteConnectionManager;
use persistent::Read as PRead;

fn main() {
    let app_path_buf = PathBuf::from("./../app/src");
    let app_path = fs::canonicalize(&app_path_buf).unwrap();
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new("../ingredients");
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let mut router = Router::new();
    router.get("/grains/:id", move |r: &mut Request| get_grains(r));

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(app_path.to_str().unwrap())));
    mount.mount("/api", router);

    let mut chain = Chain::new(mount);
    chain.link(PRead::<SqliteDB>::both(pool));

    Iron::new(chain).http("localhost:3000").unwrap();
}
