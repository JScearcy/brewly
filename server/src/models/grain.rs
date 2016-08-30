extern crate time;

use rusqlite::Error;
use services;

//#[derive(Serialize, Deserialize, Debug)]
pub struct GrainModel {
        pub id: i32,
        pub name: String,
        pub time_created: self::time::Timespec,
        pub data: String,
}

pub fn list_grains (conn: services::sqlite::SqliteConnection) -> Result<Vec<GrainModel>, Error> {
        let mut stmt = try!(conn.prepare("SELECT id, name, time_created, data from grains"));
        let rows = try!(stmt.query_map(&[], |row| {
                GrainModel {
                        id: row.get("id"),
                        name: row.get("name"),
                        time_created: row.get("time_created"),
                        data: row.get("data"),
                }
        }));
        let mut grains: Vec<GrainModel> = Vec::new();

        for row in rows {
                grains.push(row.unwrap());
        }

        Ok(grains)
}