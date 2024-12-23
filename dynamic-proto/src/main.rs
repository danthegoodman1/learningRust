use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage, Value};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This would be your proto definition in bytes (typically from a FileDescriptorSet)
    let descriptor_bytes = include_bytes!("file_descriptor_set.bin");

    // Time descriptor pool creation
    let start = Instant::now();
    let pool = DescriptorPool::decode(descriptor_bytes.as_ref())?;
    let message_descriptor = pool
        .get_message_by_name("your.package.PersonLocation")
        .unwrap();
    println!("Descriptor creation time: {:?}", start.elapsed());

    // Example: Create a new dynamic message and set its fields
    let mut example_message = DynamicMessage::new(message_descriptor.clone());
    example_message.set_field_by_name("name", Value::String("John Doe".to_string()));
    example_message.set_field_by_name("age", Value::I32(30));

    // Create and set the nested location message
    let location_descriptor = pool.get_message_by_name("your.package.Location").unwrap();
    let mut location = DynamicMessage::new(location_descriptor);
    location.set_field_by_name("lat", Value::I32(37));
    location.set_field_by_name("long", Value::I32(-122));
    example_message.set_field_by_name("location", Value::Message(location));

    // Time message encoding
    let start = Instant::now();
    let message_bytes = example_message.encode_to_vec();
    println!("Message encoding time: {:?}", start.elapsed());

    // Time message decoding
    let start = Instant::now();
    let dynamic_message = DynamicMessage::decode(message_descriptor, message_bytes.as_slice())?;
    println!("Message decoding time: {:?}", start.elapsed());

    // Time field access
    let start = Instant::now();
    if let Some(name) = dynamic_message.get_field_by_name("name") {
        println!("Name: {}", name.as_str().unwrap_or(""));
    }
    if let Some(age) = dynamic_message.get_field_by_name("age") {
        println!("Age: {}", age.as_i32().unwrap_or(0));
    }
    if let Some(location) = dynamic_message.get_field_by_name("location") {
        if let Value::Message(location_msg) = location.as_ref() {
            if let Some(lat) = location_msg.get_field_by_name("lat") {
                println!("Latitude: {}", lat.as_i32().unwrap_or(0));
            }
            if let Some(lon) = location_msg.get_field_by_name("long") {
                println!("Longitude: {}", lon.as_i32().unwrap_or(0));
            }
        }
    }
    println!("Field access time: {:?}", start.elapsed());

    Ok(())
}


// Descriptor creation time: 473.5µs
// Message encoding time: 43.875µs
// Message decoding time: 2.417µs
// Name: John Doe
// Age: 30
// Latitude: 37
// Longitude: -122
// Field access time: 27.166µs
