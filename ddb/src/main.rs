use duckdb::types::ValueRef;
use duckdb::{params, Connection};
use serde_json::{Map, Value};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<(), ()> {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE person (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  data            BLOB
                  );
        ",
    )
    .unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    let me2 = Person {
        id: 1,
        name: "Dan".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?, ?), (?, ?)",
        params![me.name, me.data, me2.name, me2.data],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?, ?)",
        params![me.name, me.data],
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let mut rows = stmt.query([]).unwrap();

    // Collect JSON objects into an array
    let mut json_array: Vec<Map<String, Value>> = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let mut json_object = serde_json::Map::new();
        let cols = row.as_ref().column_names();
        println!("cols: {:?}", cols);

        for (idx, col) in cols.iter().enumerate() {
            let value = match row.get_ref(idx).unwrap() {
                ValueRef::Null => Value::Null,
                ValueRef::Int(i) => Value::Number(serde_json::Number::from(i)),
                ValueRef::Float(r) => {
                    Value::Number(serde_json::Number::from_f64(r.into()).unwrap())
                }
                ValueRef::Text(t) => Value::String(String::from_utf8(t.to_vec()).unwrap()),
                ValueRef::Blob(b) => Value::String(String::from_utf8(b.to_vec()).unwrap()), // like b64 encode
                _ => panic!("not implemented"),
            };
            json_object.insert(col.to_string(), value);
        }

        json_array.push(json_object)
    }

    // Print the JSON array
    println!("{}", serde_json::to_string_pretty(&json_array).unwrap());

    Ok(())
}
