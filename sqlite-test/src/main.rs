/**
 * See https://sqlite.org/draft/security.html for some other limits for handling arbitrary untrusted queries
 */

use std::ffi::c_int;

use rusqlite::ffi::{sqlite3_config, sqlite3_int64, sqlite3_memory_used, sqlite3_soft_heap_limit64, SQLITE_CONFIG_MEMSTATUS, SQLITE_STATUS_MEMORY_USED};
use rusqlite::hooks::{AuthContext, Authorization};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result};
use serde_json::Map;
use serde_json::Value;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

extern "C" {
    fn sqlite3_status(op: c_int, p_current: *mut c_int, p_highwater: *mut c_int, reset_flag: c_int) -> c_int;
    pub fn sqlite3_hard_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    unsafe {
        let mut current = 0;
        let mut highwater = 0;
        let result = sqlite3_config(SQLITE_CONFIG_MEMSTATUS, 1);
        if result == 0 {
            println!("config mem status disabled");
        } else {
            println!("config mem status enabled");
        }

        let result = sqlite3_status(SQLITE_STATUS_MEMORY_USED, &mut current, &mut highwater, 0);
        if result == 0 {
            println!("Memory tracking is enabled. Current memory usage: {} bytes, Highwater mark: {} bytes", current, highwater);
        } else {
            println!("Failed to retrieve memory status.");
        }
    }

    conn.authorizer(Some(|ctx: AuthContext| {
        // println!("{:?}", ctx);
        match ctx.action {
            rusqlite::hooks::AuthAction::Insert { table_name } => {
                println!("Inserting into {}", table_name);
                if table_name == "raw_data" {
                    return Authorization::Deny;
                }
            }
            rusqlite::hooks::AuthAction::Update {
                table_name,
                column_name,
            } => {
                println!("Updating col '{}' in table '{}'", column_name, table_name);
                if table_name == "raw_data" {
                    return Authorization::Deny;
                }
            }
            rusqlite::hooks::AuthAction::DropTable { table_name } => {
                println!("Dropping table '{}'", table_name);
                if table_name == "raw_data" {
                    return Authorization::Deny;
                }
            }
            rusqlite::hooks::AuthAction::Pragma {
                pragma_name,
                pragma_value,
            } => {
                if let Some(val) = pragma_value {
                    println!(
                        "PRAGMA {} set to {}",
                        pragma_name,
                        val
                    );
                }
            }
            // _ => (), // do nothing otherwise
            _ => {
                println!("ACTION: {:?}", ctx.action)
            } // do nothing otherwise
        }
        Authorization::Allow
    }));
    // conn.authorizer::<fn(AuthContext) -> Authorization>(None); // reset to nothing

    // NOTE: Hard heap limit doesn't work on default macOS sqlite (works on linux and feature "bundled")
    // Set a very low heap limit to test the functionality
    conn.pragma_update(
        Some(rusqlite::DatabaseName::Main),
        "soft_heap_limit",
        rusqlite::types::Value::Integer(1024),
    )?;
    conn.pragma_query(Some(rusqlite::DatabaseName::Main),
    "soft_heap_limit", |row | -> Result<()> {
        println!("soft_heap_limit pragma: {:?}", row);
        Ok(())
    }).unwrap();
    conn.pragma_update(
        Some(rusqlite::DatabaseName::Main),
        "hard_heap_limit",
        rusqlite::types::Value::Integer(1024),
    )?;
    // This will likely OOM
    conn.pragma_query(Some(rusqlite::DatabaseName::Main),
    "hard_heap_limit", |row | -> Result<()> {
        println!("hard_heap_limit pragma: {:?}", row);
        Ok(())
    }).unwrap();
    // Direct ffi call method
    unsafe {
        // sqlite3_soft_heap_limit64(1024);
        let soft_limit = sqlite3_soft_heap_limit64(-1);
        println!("Soft heap limit (ffi): {}", soft_limit);
        let hard_limit = sqlite3_hard_heap_limit64(-1);
        println!("Hard heap limit (ffi): {}", hard_limit);
    }

    conn.execute(
        "CREATE TABLE person (
             id   INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             data BLOB
         )",
        (), // empty list of parameters.
    )?;

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };

    // Uncomment to test heap limits
    // for i in 0..200_000 {
    //     let result = conn.execute(
    //         "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //         (&me.name, &me.data),
    //     );

    //     match result {
    //         Ok(_) => println!("Inserted row {}", i),
    //         Err(err) => {
    //             println!("Failed to insert row {}: {}", i, err);
    //             break;
    //         }
    //     }

    //     // Check memory usage using sqlite3_memory_used FFI function
    //     let ffi_memory_used = unsafe { sqlite3_memory_used() };
    //     println!("Memory used (via FFI) after {} inserts: {} bytes", i, ffi_memory_used);
    // }

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let cols: Vec<String> = stmt.column_names().iter().map(|&s| s.to_string()).collect();
    println!("cols: {:?}", cols);
    let mut rows = stmt.query([])?;

    // Collect JSON objects into an array
    let mut json_array: Vec<Map<String, Value>> = Vec::new();

    while let Some(row) = rows.next()? {
        let mut json_object = serde_json::Map::new();

        let cols = row.as_ref().column_names();

        for (idx, col) in cols.iter().enumerate() {
            let value = match row.get_ref(idx)? {
                ValueRef::Null => Value::Null,
                ValueRef::Integer(i) => Value::Number(serde_json::Number::from(i)),
                ValueRef::Real(r) => Value::Number(serde_json::Number::from_f64(r).unwrap()),
                ValueRef::Text(t) => Value::String(String::from_utf8(t.to_vec()).unwrap()),
                ValueRef::Blob(b) => Value::String(String::from_utf8(b.to_vec()).unwrap()), // like b64 encode
            };
            json_object.insert(col.to_string(), value);
        }

        json_array.push(json_object)
    }

    // Print the JSON array
    println!("{}", serde_json::to_string_pretty(&json_array).unwrap());

    Ok(())
}
