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

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

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
            _ => (), // do nothing otherwise
        }
        Authorization::Allow
    }));
    conn.authorizer::<fn(AuthContext) -> Authorization>(None); // reset to nothing

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
