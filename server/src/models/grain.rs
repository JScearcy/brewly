extern crate rustc_serialize;

use rusqlite::Error;
use services;
// use rustc_serialize::Encodable;
// use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
pub struct GrainModel {
        pub id: i32,
        pub name: String,
        pub data: String,
}

pub fn list_grains (conn: services::sqlite::SqliteConnection) -> Result<Vec<GrainModel>, Error> {
        let mut stmt = try!(conn.prepare("SELECT id, name, time_created, data from grains"));
        let rows = try!(stmt.query_map(&[], |row| {
                GrainModel {
                        id: row.get("id"),
                        name: row.get("name"),
                        data: row.get("data"),
                }
        }));
        let mut grains: Vec<GrainModel> = Vec::new();

        for row in rows {
                grains.push(row.unwrap());
        }

        Ok(grains)
}