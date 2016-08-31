extern crate time;
extern crate iron;

use iron::typemap::Key;
use r2d2;

use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;
pub type SqliteConnection = r2d2::PooledConnection<SqliteConnectionManager>;

pub struct SqliteDB;
impl Key for SqliteDB { type Value = SqlitePool; }

// Gets a connection from the pool from the given request or returns a 500
// #[macro_export]
// macro_rules! get_sqlite_connection {
//     ($req:expr) => (match $req.get::<persistent::Read<sqlite::SqliteDB>>() {
//         Ok(pool) => match pool.get() {
//             Ok(conn) => conn,
//             Err(_) => {
//                 println!("Couldn't get a connection to Sqlite!");
//                 return Ok(Response::with((status::InternalServerError)));
//             }
//         },
//         Err(_) => {
//             println!("Couldn't get the Sqlite pool from the request!");
//             return Ok(Response::with((status::InternalServerError)));
//         }
//     })
// }

pub fn get_pool(uri: &str) -> SqlitePool {
    let manager = SqliteConnectionManager::new(uri);

    r2d2::Pool::new(r2d2::Config::default(), manager).unwrap()
}

// pub struct Sqlite {
//     pub con: Connection,
// }

// pub trait SqliteActions {
//     fn new(Connection) -> Sqlite;
//     fn insert(&self) -> &'static str;
//     fn get(&self, &str) -> Result<Vec<GrainModel>, &str>;
//     fn delete(&self) -> &'static str;
//     fn update(&self) -> &'static str;
// }

// impl SqliteActions for Sqlite {
//     fn new(c: Connection) -> Sqlite {
//         let sq_ob = Sqlite { con: c };
//         sq_ob.con.execute("CREATE TABLE grains (
//                   id              INTEGER PRIMARY KEY,
//                   name            TEXT NOT NULL,
//                   time_created    TEXT NOT NULL,
//                   data            BLOB
//                   )", &[]).unwrap();

//         let g = GrainModel {
//             id: 0,
//             name: "2-Row".to_string(),
//             time_created: time::get_time(),
//             data: "Standard 2-Row base malt".to_string(),
//         };

//         sq_ob.con.execute("INSERT INTO grains (name, time_created, data)
//                   VALUES ($1, $2, $3)",
//                  &[&g.name, &g.time_created, &g.data]).unwrap();

//         sq_ob
//     }

//     fn insert(&self) -> &'static str {
//         "Insert"
//     }

//     fn get(&self, query: & str) -> Result<Vec<GrainModel>, &str> {
//         let mut stmt = self.con.prepare("SELECT id, name, time_created, data FROM grains WHERE name = (:name)").unwrap();
//         let grains_iter = stmt.query_map_named(&[("name", &query)], |row| {
//             GrainModel {
//                 id: row.get(0),
//                 name: row.get(1),
//                 time_created: row.get(2),
//                 data: row.get(3),
//             }
//         }).unwrap();
//         let mut grains: Vec<GrainModel> = Vec::new();
//         let result: Result<Vec<GrainModel>, &str>;

//         for grain in grains_iter {
//             grains.push(grain.unwrap());
//         }

//         if grains.len() > 0 { result = Ok(grains) }
//         else { result = Err("Nothing Found!") };

//         result
//     }

//     fn delete(&self) -> &'static str {
//         "Delete"
//     }

//     fn update(&self) -> &'static str {
//         "Update"
//     }
// }