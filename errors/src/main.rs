pub mod other;

use std::fs::File;
use std::io::{self, ErrorKind, Read};

use other::{custom_top, top_level};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // let _ = read_username_from_file().unwrap_or_else(|err| {
    //     if err.kind() == ErrorKind::NotFound {
    //         println!("not found {:?}", err);
    //         panic!("panic")
    //     } else {
    //         panic!("other error!")
    //     }
    // });

    // let _ = match read_username_from_file() {
    //     Ok(_) => {},
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => {
    //             println!("not found {:?}", err);
    //         },
    //         other => {
    //             println!("got error {:?}", err);
    //         }
    //     }
    // };

    // Demo with anyhow getting errors
    match top_level() {
        Err(e) => {
            // println!("got error {:?}", e);
            // Check the error type
            match e.downcast_ref() {
                Some(ErrorKind::NotFound) => {
                    println!("it was not found:\n{}", e.chain().enumerate().map(|(index, cause)| format!("\t{}: {}", index, cause.to_string())).collect::<Vec<String>>().join("\n"));
                    // println!("it was not found: {}", e);
                },
                Some(&_) => {
                    // tf is this?
                }
                None => {
                    println!("it was something else!")
                }
            }
        },
        _other => (),
    }

    println!("_____________");
    
    match custom_top() {
        Err(e) => {
            // println!("got error: {:?}", e);
            println!("got the error:\n{}", e.chain().enumerate().map(|(index, cause)| format!("\t{}: {}", index, cause.to_string())).collect::<Vec<String>>().join("\n"));
        },
        _other => (),
    }
}
