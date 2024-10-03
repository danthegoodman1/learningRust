use prost::Message;
use prost_reflect::prost_types::{
    field_descriptor_proto, DescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet,
};
use prost_reflect::{DescriptorPool, DynamicMessage, ReflectMessage};
use serde_json::Value;
use std::error::Error;

fn json_schema_to_file_descriptor_set(
    schema_str: &str,
) -> Result<FileDescriptorSet, Box<dyn Error>> {
    let schema: Value = serde_json::from_str(schema_str)?;

    let message_name = schema["name"].as_str().ok_or("Missing message name")?;
    let fields = schema["field"].as_array().ok_or("Missing fields array")?;

    let mut proto_fields = Vec::new();
    for field in fields {
        let name = field["name"]
            .as_str()
            .ok_or("Missing field name")?
            .to_string();
        let number = field["number"].as_i64().ok_or("Missing field number")? as i32;
        let field_type = match field["type"].as_str().ok_or("Missing field type")? {
            "TYPE_DOUBLE" => field_descriptor_proto::Type::Double,
            "TYPE_FLOAT" => field_descriptor_proto::Type::Float,
            "TYPE_INT64" => field_descriptor_proto::Type::Int64,
            "TYPE_UINT64" => field_descriptor_proto::Type::Uint64,
            "TYPE_INT32" => field_descriptor_proto::Type::Int32,
            "TYPE_FIXED64" => field_descriptor_proto::Type::Fixed64,
            "TYPE_FIXED32" => field_descriptor_proto::Type::Fixed32,
            "TYPE_BOOL" => field_descriptor_proto::Type::Bool,
            "TYPE_STRING" => field_descriptor_proto::Type::String,
            "TYPE_GROUP" => field_descriptor_proto::Type::Group,
            "TYPE_MESSAGE" => field_descriptor_proto::Type::Message,
            "TYPE_BYTES" => field_descriptor_proto::Type::Bytes,
            "TYPE_UINT32" => field_descriptor_proto::Type::Uint32,
            "TYPE_ENUM" => field_descriptor_proto::Type::Enum,
            "TYPE_SFIXED32" => field_descriptor_proto::Type::Sfixed32,
            "TYPE_SFIXED64" => field_descriptor_proto::Type::Sfixed64,
            "TYPE_SINT32" => field_descriptor_proto::Type::Sint32,
            "TYPE_SINT64" => field_descriptor_proto::Type::Sint64,
            _ => return Err("Unsupported field type".into()),
        };

        // Handle repeating fields in proto3
        let label = if field["label"].as_str() == Some("LABEL_REPEATED") {
            field_descriptor_proto::Label::Repeated
        } else {
            field_descriptor_proto::Label::Optional // In proto3, all fields are implicitly optional
        };

        proto_fields.push(FieldDescriptorProto {
            name: Some(name),
            number: Some(number),
            label: Some(label as i32),
            r#type: Some(field_type as i32),
            ..Default::default()
        });
    }

    let file_descriptor_proto = FileDescriptorProto {
        name: Some("dynamic.proto".to_string()),
        package: Some("".to_string()),
        syntax: Some("proto3".to_string()),
        message_type: vec![DescriptorProto {
            name: Some(message_name.to_string()),
            field: proto_fields,
            ..Default::default()
        }],
        ..Default::default()
    };

    Ok(FileDescriptorSet {
        file: vec![file_descriptor_proto],
    })
}

fn dynamic_protobuf_deserialize(
    file_descriptor_set: FileDescriptorSet,
    message_name: &str,
    serialized_data: &[u8],
) -> Result<DynamicMessage, Box<dyn Error>> {
    // Create a new DescriptorPool
    let mut pool = DescriptorPool::new();

    // Add the file descriptor set to the pool
    pool.add_file_descriptor_set(file_descriptor_set)?;

    // Get the message descriptor
    let message_descriptor = pool
        .get_message_by_name(message_name)
        .ok_or("Message not found in the protobuf definition")?;

    // Create a dynamic message
    let mut dynamic_message = DynamicMessage::new(message_descriptor.clone());

    // Deserialize the data
    dynamic_message.merge(serialized_data)?;

    Ok(dynamic_message)
}

fn main() -> Result<(), Box<dyn Error>> {
    let schema_str = r#"{
        "name": "SearchRequest",
        "field": [
            {"name": "query", "number": 1, "type": "TYPE_STRING"},
            {"name": "page_number", "number": 2, "type": "TYPE_INT32"},
            {"name": "results_per_page", "number": 3, "type": "TYPE_INT32"},
            {"name": "scores", "number": 4, "label": "LABEL_REPEATED", "type": "TYPE_INT32"}
        ]
    }"#;

    let file_descriptor_set = json_schema_to_file_descriptor_set(schema_str)?;

    // Updated serialized data to include the repeating field
    let serialized_data = vec![
        10, 5, 104, 101, 108, 108, 111, // query: "hello"
        16, 2,                          // page_number: 2
        24, 10,                         // results_per_page: 10
        34, 6, 10, 20, 30, 40, 50, 60   // scores: [10, 20, 30, 40, 50, 60]
    ];

    let dynamic_message =
        dynamic_protobuf_deserialize(file_descriptor_set, "SearchRequest", &serialized_data)?;

    // Access fields dynamically
    println!(
        "Query: {}",
        dynamic_message.get_field_by_name("query").unwrap().as_str().unwrap()
    );
    println!(
        "Page Number: {}",
        dynamic_message.get_field_by_name("page_number").unwrap().as_ref()
    );
    println!(
        "Results Per Page: {}",
        dynamic_message
            .get_field_by_name("results_per_page")
            .unwrap()
            .as_ref()
    );

    // Access and print the repeating field
    if let Some(scores) = dynamic_message.get_field_by_name("scores") {
        println!("Scores: {:?}", scores.as_list().unwrap());
    }

    // Log the type of each field by name
    for field in dynamic_message.descriptor().fields() {
        let field_name = field.name();
        let field_value = dynamic_message.get_field_by_name(field_name).unwrap();
        let fieldreal = dynamic_message.descriptor().get_field_by_name(field_name).unwrap();

        if let Some(list) = field_value.as_list() {
            println!("Field '{}' is list of kind: {:?}", field_name, fieldreal.kind());
            for item in list {
                println!("\t{:?}", item);
            }
        } else {
            println!("Field '{}' is of kind: {:?}", field_name, fieldreal.kind());
            println!("Field '{}' value: {:?}", field_name, field_value);
        }
    }

    Ok(())
}
