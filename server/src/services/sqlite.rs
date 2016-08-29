extern crate rusqlite;
extern crate time;
extern crate iron;

use self::rusqlite::Connection;
use models::grain::GrainModel;
use iron::typemap::Key;

pub struct PersistentSqlite {}
impl Key for PersistentSqlite { type Value = Sqlite; }

pub struct Sqlite {
    pub con: Connection,
}

pub trait SqliteActions {
    fn new(Connection) -> Sqlite;
    fn insert(&self) -> &'static str;
    fn get(&self, &str) -> Result<Vec<GrainModel>, &str>;
    fn delete(&self) -> &'static str;
    fn update(&self) -> &'static str;
}

impl SqliteActions for Sqlite {
    fn new(c: Connection) -> Sqlite {
        let sq_ob = Sqlite { con: c };
        sq_ob.con.execute("CREATE TABLE grains (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();

        let g = GrainModel {
            id: 0,
            name: "2-Row".to_string(),
            time_created: time::get_time(),
            data: "Standard 2-Row base malt".to_string(),
        };

        sq_ob.con.execute("INSERT INTO person (name, time_created, data)
                  VALUES ($1, $2, $3)",
                 &[&g.name, &g.time_created, &g.data]).unwrap();

        sq_ob
    }

    fn insert(&self) -> &'static str {
        "Insert"
    }

    fn get(&self, query: & str) -> Result<Vec<GrainModel>, &str> {
        let mut stmt = self.con.prepare("SELECT id, name, time_created, data FROM grains WHERE name = (:name)").unwrap();
        let mut grains = stmt.query_map_named(&[("name", &query)], |x| x).unwrap();
        let mut found_grains: Vec<GrainModel> = Vec::new();
        let mut result: Result<Vec<GrainModel>, &str> = Err("Nothing found!");

        for grain_result in grains {

            let grn_row: GrainModel = grain_result.unwrap().get(0);
            let grn = GrainModel {
                id: grn_row.id,
                name: grn_row.name,
                time_created: grn_row.time_created,
                data: grn_row.data,
            };
            found_grains.push(grn);
        };
        if found_grains.len() > 0 {
            result = Ok(found_grains);
        };
        result
    }

    fn delete(&self) -> &'static str {
        "Delete"
    }

    fn update(&self) -> &'static str {
        "Update"
    }
}