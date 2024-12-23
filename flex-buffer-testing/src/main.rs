// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use flexbuffers;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Weapon {
    Fist,
    Equipment { name: String, damage: i32 },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Color(u8, u8, u8, u8);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Monster {
    hp: u32,
    mana: i32,
    enraged: bool,
    weapons: Vec<Weapon>,
    color: Color,
    position: [f64; 3],
    velocity: [f64; 3],
    coins: Vec<u32>,
}

fn main() {
    let monster = Monster {
        hp: 80,
        mana: 200,
        enraged: true,
        color: Color(255, 255, 255, 255),
        position: [0.0; 3],
        velocity: [1.0, 0.0, 0.0],
        weapons: vec![
            Weapon::Fist,
            Weapon::Equipment {
                name: "great axe".to_string(),
                damage: 15,
            },
            Weapon::Equipment {
                name: "hammer".to_string(),
                damage: 5,
            },
        ],
        coins: vec![500; 100],
    };

    // FlexBuffers serialization
    let start_flex_ser = std::time::Instant::now();
    let mut flex_buf = flexbuffers::FlexbufferSerializer::new();
    monster.serialize(&mut flex_buf).unwrap();
    let flex_ser_time = start_flex_ser.elapsed();
    
    // FlexBuffers deserialization
    let flex_data = flex_buf.view();
    let start_flex_deser = std::time::Instant::now();
    let r = flexbuffers::Reader::get_root(flex_data).unwrap();
    let monster2 = Monster::deserialize(r).unwrap();
    let flex_deser_time = start_flex_deser.elapsed();

    // JSON serialization
    let start_json_ser = std::time::Instant::now();
    let json_data = serde_json::to_vec(&monster).unwrap();
    let json_ser_time = start_json_ser.elapsed();

    // JSON deserialization
    let start_json_deser = std::time::Instant::now();
    let monster3: Monster = serde_json::from_slice(&json_data).unwrap();
    let json_deser_time = start_json_deser.elapsed();

    // Add direct field access test for FlexBuffers
    let start_flex_access = std::time::Instant::now();
    let r = flexbuffers::Reader::get_root(flex_data).unwrap();
    let hp = r.as_map().idx("hp").as_u32();
    let flex_access_time = start_flex_access.elapsed();
    
    println!("FlexBuffers direct access: {:?}", flex_access_time);

    println!("Comparison:");
    println!("FlexBuffers size: {} bytes", flex_data.len());
    println!("JSON size: {} bytes", json_data.len());
    println!("\nTiming:");
    println!("FlexBuffers serialization: {:?}", flex_ser_time);
    println!("FlexBuffers deserialization: {:?}", flex_deser_time);
    println!("JSON serialization: {:?}", json_ser_time);
    println!("JSON deserialization: {:?}", json_deser_time);

    // Print FlexBuffer hex representation
    println!("\nFlexBuffer hex representation:");
    for (i, byte) in flex_data.iter().enumerate() {
        print!("{:02x}", byte);
        if (i + 1) % 16 == 0 {
            println!();  // New line every 16 bytes
        } else {
            print!(" ");
        }
    }
    println!("\n");  // Extra newline at the end

    assert_eq!(monster, monster2);
    assert_eq!(monster, monster3);
}

// FlexBuffers direct access: 375ns
// Comparison:
// FlexBuffers size: 427 bytes
// JSON size: 630 bytes

// Timing:
// FlexBuffers serialization: 57.625µs
// FlexBuffers deserialization: 51.25µs
// JSON serialization: 3.584µs
// JSON deserialization: 6.666µs
