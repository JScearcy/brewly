extern crate rustc_serialize;

use rusqlite::Error;
use services;
use self::rustc_serialize::{Encodable, Decodable, Decoder, Encoder};

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

impl Encodable for GrainModel {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("GrainModel", 3, |s| {
            try!(s.emit_struct_field("id", 0, |s| {
                s.emit_i32(self.id)
            }));
            try!(s.emit_struct_field("name", 1, |s| {
                s.emit_str(&self.name)
            }));
            try!(s.emit_struct_field("data", 1, |s| {
                s.emit_str(&self.data)
            }));
            Ok(())
        })
    }
}

impl Decodable for GrainModel {
    fn decode<D: Decoder>(d: &mut D) -> Result<GrainModel, D::Error> {
        d.read_struct("GrainModel", 3, |d| {
            let id = try!(d.read_struct_field("id", 0, |d| { d.read_i32() }));
            let name = try!(d.read_struct_field("name", 1, |d| { d.read_str() }));
            let data = try!(d.read_struct_field("data", 1, |d| { d.read_str() }));
            Ok(GrainModel{ id: id, name: name, data: data })
        })
    }
}