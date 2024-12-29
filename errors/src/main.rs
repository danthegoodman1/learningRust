pub mod chain;
pub mod other;

use std::fs::File;
use std::io::{self, ErrorKind, Read};

use chain::{operation_a, operation_root, CustomErrorA, CustomErrorB, CustomErrorC};
use other::{custom_top, top_level};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn do_something_wrong() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("something went wrong").context("from do_something_wrong"))
}

fn main() {
    match do_something_wrong() {
        Ok(_) => (),
        Err(e) => {
            println!("got error: {:?}", e.root_cause());
        }
    }
    println!("_____________");
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
                    println!(
                        "it was not found:\n{}",
                        e.chain()
                            .enumerate()
                            .map(|(index, cause)| format!("\t{}: {}", index, cause.to_string()))
                            .collect::<Vec<String>>()
                            .join("\n")
                    );
                    // println!("it was not found: {}", e);
                }
                Some(&_) => {
                    // tf is this?
                }
                None => {
                    println!("it was something else!")
                }
            }
        }
        _other => (),
    }

    println!("_____________");

    match custom_top() {
        Err(e) => {
            // println!("got error: {:?}", e);
            println!(
                "got the error:\n{}",
                e.chain()
                    .enumerate()
                    .map(|(index, cause)| format!("\t{}: {}", index, cause.to_string()))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
        _other => (),
    }

    let result = operation_root();

    if let Err(err) = result {
        // println!(
        //     "Err chain on operation_root:\n{}",
        //     err.chain()
        //         .enumerate()
        //         .map(|(index, cause)| format!("\t{}: {}", index, cause.to_string()))
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );
        eprintln!("Err chain on operation_root: {:?}", err);

        if err.downcast_ref::<CustomErrorA>().is_some() {
            println!("Error chain contains CustomErrorA");
        }
        if err.downcast_ref::<CustomErrorB>().is_some() {
            println!("Error chain contains CustomErrorB");
        }
        if err.downcast_ref::<CustomErrorC>().is_some() {
            println!("Error chain contains CustomErrorC");
        }

        // You can also print the full error chain
        eprintln!("Full error chain: {:#}", err);
    }

    // check out https://docs.rs/color-eyre/latest/color_eyre/ too for better span traces (would need to disable the parameter printing)
}
