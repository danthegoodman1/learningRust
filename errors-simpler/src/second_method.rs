use std::fmt::Display;

use anyhow::{Context, Result};
use thiserror::Error;

pub struct Location {
    file: &'static str,
    line: u32,
    column: u32,
}

pub trait ErrorLocation<T, E> {
    fn location_with_fn_name(self, fn_name: &str, loc: &'static Location) -> Result<T>;
}


impl<T, E> ErrorLocation<T, E> for Result<T, E>
where
    E: Display,
    Result<T, E>: Context<T, E>,
{
    fn location_with_fn_name(self, fn_name: &str, loc: &'static Location) -> Result<T> {
        self.with_context(|| format!(
            "{}()::{}:{}:{}",
            fn_name,
            loc.file, loc.line, loc.column,
        ))
    }
}

macro_rules! here {
    () => {
        &Location {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

#[derive(Error, Debug)]
enum MyErrors {
    #[error("Custom error")]
    CustomError,
}

fn bottom() -> Result<()> {
    Err(MyErrors::CustomError).location_with_fn_name("bottom", here!())
}

fn middle() -> Result<()> {
    bottom().location_with_fn_name("middle", here!())?;
    Ok(())
}

fn top() -> Result<()> {
    middle().location_with_fn_name("top", here!())?;
    Ok(())
}

pub fn second_main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    match top() {
        Ok(_) => println!("Success"),
        Err(e) => {
            println!("Error: {}", e);
            println!("\nError chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("  {}: {}", i, cause);
            }

            println!("\nBacktrace:");
            println!("{}", e.backtrace());
        }
    }
}
