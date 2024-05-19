use duckdb::{Connection, params};
use std::any::Any;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde_json::Value;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE person (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  data            BLOB
                  );
        ").unwrap();

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
    ).unwrap();

    // Write it out to json file
    conn.execute("copy (SELECT id, name, data FROM person) to 'out.ndjson'", []).unwrap();

    // Read in the json line by line
    let path = Path::new("out.ndjson");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(&line).unwrap();
        for obj in v.as_object().unwrap() {
            println!("- s: {}\n  v: {}", obj.0, obj.1);
            match obj.1 {
                Value::Number(n) => {
                    println!("is number {}", n)
                },
                Value::Null => {
                    println!("is null")
                },
                Value::String(s) => {
                    println!("is str {}", s)
                },
                Value::Array(a) => {
                    println!("is arr")
                },
                Value::Object(o) => {
                    println!("is obj")
                },
                Value::Bool(b) => {
                    println!("is bool {}", b)
                },
                _ => {
                    println!("Is a {}", obj.1)
                }
            }
        }

        // Access parts of the document like with a HashMap.
        println!("{}", v);
    }


    // println!("cols: {:?}", cols);
    // let person_iter = result.mapped(|row| {
    //     let mut hm: HashMap<String, Box<dyn Any>> = HashMap::new();
    //     for (i, col) in cols.iter().enumerate() {
    //         let val = row.get_ref(i).unwrap();
    //         hm.insert(col.clone(), Box::new(val.to_owned()));
    //     }
    //     Ok(hm)
    // });

    // for person in person_iter {
    //     println!("Found person {:?}", person);
    // }
}
