fn main() {
    println!("Listening!");
    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

use rdev::{listen, Event, Key};
use test_rdev::keycodes::code_from_key;

fn callback(event: Event) {
    match event.event_type {
        rdev::EventType::KeyPress(key) => println!("User wrote {:?} ({:?})", key, code_from_key(key)),
        _ => (),
    }
}
