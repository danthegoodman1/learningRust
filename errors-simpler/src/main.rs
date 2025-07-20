
use errors_simpler::{first_method::first_main, second_method::second_main};


fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    first_main();

    println!("--------------------------------");

    second_main();

}
