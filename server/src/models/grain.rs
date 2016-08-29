extern crate time;
extern crate rusqlite;

use rusqlite::types::{FromSql, sqlite3_stmt};
use std::os::raw::c_int;

pub struct GrainModel {
        pub id: i32,
        pub name: String,
        pub time_created: self::time::Timespec,
        pub data: String,
}

impl FromSql for GrainModel {
        unsafe fn column_result (stmt: *mut sqlite3_stmt, col: c_int) -> Result<Self, &str> {
                let mut grn: GrainModel;
                if stmt != null {
                        grn = GrainModel {
                                id: stmt.id,
                                name: stmt.name,
                                time_created: stmt.time_created,
                                data: stmt.data
                        };
                }
                if grn != null { return Ok(grn)}
                else { return Err("Not Found") }
        }
}