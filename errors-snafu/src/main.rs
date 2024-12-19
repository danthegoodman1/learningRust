use errors_snafu::{file_invalid, generic_error, invalid_id, invalid_name, InvalidIdError};
use snafu::{whatever, Whatever};

fn whatever_example() -> Result<(), Whatever> {
    println!("Whatever example");
    whatever!("This is a test");
}

fn invalid_id_example() -> Result<(), InvalidIdError> {
    let id = 1;
    invalid_id(id)
}

fn main() {
    // Simple with backtrace
    match whatever_example() {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}\n{}", e, e.backtrace().unwrap()),
    }

    println!("\n\nInvalid id example");

    // Custom error
    match invalid_id_example() {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}\nLocation: {}", e, e.location),
    }

    println!("\n\nFile invalid example");

    // Custom error with context
    match file_invalid("test.txt") {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}\nLocation: {}", e, e.location),
    }

    println!("\n\nInvalid name example");

    // Custom error with context
    match invalid_name("Da") {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n\nGeneric error example");

    // Generic error
    match generic_error() {
        Ok(_) => println!("Success"),
        Err(e) => match e {
            errors_snafu::Error::InvalidName { location, .. } => println!("Error: {}\nLocation: {}", e, location),
            _ => println!("Error: {}", e),
        },
    }
}
