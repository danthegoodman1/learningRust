use anyhow::{Context, Result};
use thiserror::Error;

// Macro that takes function name as parameter
macro_rules! loc_with_fn_name {
    ($fn_name:literal) => {
        format!("{}()::{}:{}:{}", $fn_name, file!(), line!(), column!())
    };
}

#[derive(Error, Debug)]
enum MyErrors {
    #[error("Custom error")]
    CustomError,
}

fn bottom() -> Result<()> {
    Err(MyErrors::CustomError).with_context(|| loc_with_fn_name!("bottom"))
}

fn middle() -> Result<()> {
    bottom().with_context(|| loc_with_fn_name!("middle"))?;
    Ok(())
}

fn top() -> Result<()> {
    middle().with_context(|| loc_with_fn_name!("top"))?;
    Ok(())
}

pub fn first_main() {
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
