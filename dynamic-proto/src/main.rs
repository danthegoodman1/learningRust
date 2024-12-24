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

    // Time complete operation: bytes -> value access
    let start = Instant::now();

    // Step 1: Create descriptor and get message type
    let pool = DescriptorPool::decode(descriptor_bytes.as_ref())?;
    let message_descriptor = pool
        .get_message_by_name("your.package.PersonLocation")
        .unwrap();

    // Step 2: Decode the message
    let dynamic_message = DynamicMessage::decode(message_descriptor, message_bytes.as_slice())?;

    // Step 3: Access a field (e.g., name)
    if let Some(name) = dynamic_message.get_field_by_name("name") {
        let _name_str = name.as_str().unwrap_or("");
    }

    println!(
        "Complete operation (bytes -> value access): {:?}",
        start.elapsed()
    );

    analyze_monster(descriptor_bytes)?;

    Ok(())
}

fn analyze_monster(descriptor_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Create pool and get descriptors
    let pool = DescriptorPool::decode(descriptor_bytes)?;
    let monster_descriptor = pool.get_message_by_name("your.package.Monster").unwrap();
    let weapon_descriptor = pool.get_message_by_name("your.package.Weapon").unwrap();
    let equipment_descriptor = pool.get_message_by_name("your.package.Equipment").unwrap();
    let color_descriptor = pool.get_message_by_name("your.package.Color").unwrap();

    println!("Creating monster message...");
    let start = Instant::now();

    // Create the monster message
    let mut monster = DynamicMessage::new(monster_descriptor.clone());
    monster.set_field_by_name("hp", Value::U32(80));
    monster.set_field_by_name("mana", Value::I32(200));
    monster.set_field_by_name("enraged", Value::Bool(true));

    // Set color
    let mut color = DynamicMessage::new(color_descriptor);
    color.set_field_by_name("r", Value::U32(255));
    color.set_field_by_name("g", Value::U32(255));
    color.set_field_by_name("b", Value::U32(255));
    color.set_field_by_name("a", Value::U32(255));
    monster.set_field_by_name("color", Value::Message(color));

    // Set position and velocity
    monster.set_field_by_name(
        "position",
        Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
    );
    monster.set_field_by_name(
        "velocity",
        Value::List(vec![Value::F64(1.0), Value::F64(0.0), Value::F64(0.0)]),
    );

    // Create weapons
    let weapons = vec![
        {
            let mut weapon = DynamicMessage::new(weapon_descriptor.clone());
            weapon.set_field_by_name("fist", Value::Bool(true));
            weapon
        },
        {
            let mut weapon = DynamicMessage::new(weapon_descriptor.clone());
            let mut equipment = DynamicMessage::new(equipment_descriptor.clone());
            equipment.set_field_by_name("name", Value::String("great axe".to_string()));
            equipment.set_field_by_name("damage", Value::I32(15));
            weapon.set_field_by_name("equipment", Value::Message(equipment));
            weapon
        },
        {
            let mut weapon = DynamicMessage::new(weapon_descriptor.clone());
            let mut equipment = DynamicMessage::new(equipment_descriptor.clone());
            equipment.set_field_by_name("name", Value::String("hammer".to_string()));
            equipment.set_field_by_name("damage", Value::I32(5));
            weapon.set_field_by_name("equipment", Value::Message(equipment));
            weapon
        },
    ];
    monster.set_field_by_name(
        "weapons",
        Value::List(weapons.into_iter().map(Value::Message).collect()),
    );

    // Set coins
    monster.set_field_by_name("coins", Value::List(vec![Value::U32(500); 100]));

    println!("Monster creation time: {:?}", start.elapsed());

    // Measure encoding
    let start = Instant::now();
    let message_bytes = monster.encode_to_vec();
    let encoding_time = start.elapsed();
    println!("Message size: {} bytes", message_bytes.len());
    println!("Encoding time: {:?}", encoding_time);

    // Measure decoding
    let start = Instant::now();
    let decoded_monster =
        DynamicMessage::decode(monster_descriptor.clone(), message_bytes.as_slice())?;
    let decoding_time = start.elapsed();
    println!("Decoding time: {:?}", decoding_time);

    // Measure coin access
    let start = Instant::now();
    if let Some(coins) = decoded_monster.get_field_by_name("coins") {
        if let Some(coin_list) = coins.as_list() {
            if let Some(first_coin) = coin_list.first() {
                println!("First coin value: {}", first_coin.as_u32().unwrap_or(0));
            }
        }
    }
    println!("Coin access time: {:?}", start.elapsed());

    Ok(())
}

// Descriptor creation time: 461.5µs
// Message encoding time: 58.083µs
// Message decoding time: 2.792µs
// Name: John Doe
// Age: 30
// Latitude: 37
// Longitude: -122
// Field access time: 25.459µs
// Complete operation (bytes -> value access): 48.583µs
// Creating monster message...
// Monster creation time: 10.917µs
// Message size: 311 bytes
// Encoding time: 13.375µs
// Decoding time: 13.167µs
// First coin value: 500
// Coin access time: 1.583µs

// Related:
// FlexBuffers direct access: 167ns
// Comparison:
// FlexBuffers size: 427 bytes
// JSON size: 630 bytes

// Timing:
// FlexBuffers serialization: 35.75µs
// FlexBuffers deserialization: 27.208µs
// JSON serialization: 1.708µs
// JSON deserialization: 2.875µs
