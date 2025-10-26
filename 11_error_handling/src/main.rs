// Demonstration of Option and Result types in Rust
// Option is used when a value can be either something or nothing
// Result is used for operations that can succeed or fail
fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure!".to_string())
    }
}

// Result type to represent success or failure of an operation
// Result is not the same as Option because the return value is type not differentiated.
fn open_door(has_key: bool) -> Result<String, String> {
    if has_key {
        Ok("The door is now open.".to_string())
    } else {
        Err("You need a key to open the door.".to_string())
    }
}

fn main() {
    let _chest_result: String = match open_chest(false) {
        Some(message) =>  message,
        None => "The chest is empty.".to_string(),
    };
    println!("Chest Result: {}", _chest_result);
    let _door_result: String = match open_door(true) {
        Ok(message) => message,
        Err(error) => error,
    };
    println!("Door Result: {}", _door_result);
}
