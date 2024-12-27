use std::collections::HashMap;
use std::sync::Mutex;

use redcon_learning::parser::parse_set_command;

fn main() {
    let db: Mutex<HashMap<Vec<u8>, Vec<u8>>> = Mutex::new(HashMap::new());

    let mut s = redcon::listen("127.0.0.1:6380", db).unwrap();
    s.command = Some(|conn, db, args| {
        let name = String::from_utf8_lossy(&args[0]).to_lowercase();
        match name.as_str() {
            "ping" => conn.write_string("PONG"),
            "set" => {
                if args.len() < 3 {
                    conn.write_error("ERR wrong number of arguments");
                    return;
                }
                print_vec_vec_u8(&args);
                let mut db = db.lock().unwrap();
                db.insert(args[1].to_owned(), args[2].to_owned());
                // conn.write_string("OK");
                let parsed = parse_set_command(
                    &args[1..]
                        .into_iter()
                        .map(|v| String::from_utf8_lossy(&v).to_string())
                        .collect::<Vec<_>>(),
                );
                println!("parsed: {:?}", parsed);
                conn.write_bulk("msg".as_bytes());
            }
            "get" => {
                if args.len() < 2 {
                    conn.write_error("ERR wrong number of arguments");
                    return;
                }
                print_vec_vec_u8(&args);
                let db = db.lock().unwrap();
                match db.get(&args[1]) {
                    Some(val) => conn.write_bulk(val),
                    None => conn.write_null(),
                }
            }
            _ => conn.write_error("ERR unknown command"),
        }
    });
    println!("Serving at {}", s.local_addr());
    s.serve().unwrap();
}

// Prints the Vec<Vec<u8>> as a string
fn print_vec_vec_u8(vec: &Vec<Vec<u8>>) {
    let strings = vec
        .iter()
        .map(|v| String::from_utf8_lossy(v))
        .collect::<Vec<_>>();
    println!("{:?}", strings);
}

#[cfg(test)]
mod tests {
    use redis::Commands;

    #[test]
    fn test_redis_connection() -> redis::RedisResult<()> {
        // Connect to Redis
        let client = redis::Client::open("redis://127.0.0.1:6380/")?;
        let mut con = client.get_connection()?;

        // Test set and get operations
        let _: () = con.set("my_key", 42)?;
        let value: isize = con.get("my_key")?;

        println!("value: {}", value);
        assert_eq!(value, 42);
        Ok(())
    }

    #[test]
    fn test_complex_set_command() -> redis::RedisResult<()> {
        // Connect to Redis
        let client = redis::Client::open("redis://127.0.0.1:6380/")?;
        let mut con = client.get_connection()?;

        // Test set and get operations
        let opts = redis::SetOptions::default()
            .conditional_set(redis::ExistenceCheck::NX)
            .get(true)
            .with_expiration(redis::SetExpiry::EX(60));
        let _: () = con.set_options("my_key", 42, opts)?;
        let value: isize = con.get("my_key")?;

        println!("value: {}", value);
        assert_eq!(value, 42);
        Ok(())
    }
}
