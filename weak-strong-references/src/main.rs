use std::sync::{Arc, Weak};

fn main() {
    // Create an Arc that owns the value.
    let strong = Arc::new(42);

    // Create a Weak pointer from the Arc.
    let weak1: Weak<i32> = Arc::downgrade(&strong);

    // Clone the weak pointer to create another weak reference.
    let weak2 = weak1.clone();

    // Both weak1 and weak2 refer to the same allocation.
    if let Some(value) = weak1.upgrade() {
        println!("weak1 upgraded: {}", *value);
    } else {
        println!("weak1: The value has been dropped!");
    }

    if let Some(value) = weak2.upgrade() {
        println!("weak2 upgraded: {}", *value);
    } else {
        println!("weak2: The value has been dropped!");
    }

    // Drop the strong reference.
    drop(strong);

    // Now, upgrading either weak pointer will return None.
    if let Some(_) = weak1.upgrade() {
        println!("After drop: weak1 still has access.");
    } else {
        println!("After drop: weak1 reports the value is gone.");
    }

    if let Some(_) = weak2.upgrade() {
        println!("After drop: weak2 still has access.");
    } else {
        println!("After drop: weak2 reports the value is gone.");
    }
}
