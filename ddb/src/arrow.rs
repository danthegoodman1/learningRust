use arrow::array::{Array, AsArray, Datum};
use arrow::datatypes::DataType;
use arrow::ipc::Utf8;
use duckdb::{Connection, params};
use serde_json::{Value, Map};
use std::any::Any;
use std::collections::HashMap;
use arrow::record_batch::RecordBatch;

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
                  id              INT8 PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  data            BLOB,
                  blah BOOL NOT NULL DEFAULT FALSE
                  );
        ").unwrap();

    let me = Person {
        id: 0, // seq
        name: "Steven".to_string(),
        data: None,
    };
    let me2 = Person {
        id: 0,
        name: "Dan".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?, ?), (?, ?)",
        params![me.name, me.data, me2.name, me2.data],
    ).unwrap();

    // Write it out to json file
    let mut stmt = conn.prepare("SELECT id, name, data, blah FROM person").unwrap();
    let result: Vec<RecordBatch> = stmt.query_arrow([]).unwrap().collect();
    println!("RESULT: {:?}", result);
    let numrows = result[0].num_rows();
    let mut json_arr: Vec<Map<String, Value>> = Vec::with_capacity(numrows);
    println!("Num rows {:?}, arr len {:?}", numrows, json_arr.len());
    for field in result[0].schema().fields() {
        println!("FIELD: {:?} {:?}", field.name(), field.data_type())
    }
    for (coli, col) in result[0].columns().iter().enumerate() {
        let key = result[0].schema().fields()[coli].name().clone();
        println!("COL: {:?}", key);
        match col.data_type() {
            DataType::Int64 => {
                for (i, v) in col.as_any().downcast_ref::<arrow::array::Int64Array>().unwrap().iter().enumerate() {
                    let val: Value;
                    match v {
                        None => {
                            val = Value::Null
                        }
                        Some(num) => {
                            val = Value::Number(serde_json::Number::from(num))
                        }
                    }
                    // Check if the map exists yet
                    if json_arr.len() == i {
                        // We need to append a map
                        json_arr.push(Map::new());
                    }

                    json_arr[i].insert(key.clone(), val);
                }

            }
            DataType::Utf8 => {
                for (i, v) in col.as_any().downcast_ref::<arrow::array::StringArray>().unwrap().iter().enumerate() {
                    let val: Value;
                    match v {
                        None => {
                            val = Value::Null
                        }
                        Some(str) => {
                            val = Value::String(String::from(str))
                        }
                    }
                    // Check if the map exists yet
                    if json_arr.len() == i {
                        // We need to append a map
                        json_arr.push(Map::new());
                    }

                    json_arr[i].insert(key.clone(), val);
                }
            }
            DataType::Binary => {
                for (i, v) in col.as_any().downcast_ref::<arrow::array::BinaryArray>().unwrap().iter().enumerate() {
                    let val: Value;
                    match v {
                        None => {
                            val = Value::Null
                        }
                        Some(b) => {
                            val = Value::Null
                            // TODO: base64 encode
                            // val = Value::String(String::from(b))
                        }
                    }
                    // Check if the map exists yet
                    if json_arr.len() == i {
                        // We need to append a map
                        json_arr.push(Map::new());
                    }

                    json_arr[i].insert(key.clone(), val);
                }
            }
            DataType::Boolean => {
                for (i, v) in col.as_any().downcast_ref::<arrow::array::BooleanArray>().unwrap().iter().enumerate() {
                    let val: Value;
                    match v {
                        None => {
                            val = Value::Null
                        }
                        Some(b) => {
                            val = Value::Bool(b)
                        }
                    }
                    // Check if the map exists yet
                    if json_arr.len() == i {
                        // We need to append a map
                        json_arr.push(Map::new());
                    }

                    json_arr[i].insert(key.clone(), val);
                }
            }
            _other => {
                // Other data types
            }
        }
    }

    for row in json_arr {
        println!("{:?}", serde_json::to_string(&row).unwrap())
    }


    // for col in result[0].columns().iter() {

    //     println!("Col: {:?} {:?}", col.data_type(), result[0].schema())
    // }

    // let cols: Vec<String> = Vec::new();
    // {
    //     let items = stmt.column_names();
    // }

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
