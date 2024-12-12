use scylla::{
    frame::response::result::ColumnType,
    serialize::row::SerializedValues,
    transport::partitioner::{calculate_token_for_partition_key, Murmur3Partitioner},
};

fn main() {
    let partitioner = Murmur3Partitioner {};

    let mut serialized_pk = SerializedValues::new();
    serialized_pk
        .add_value(&"hello", &ColumnType::Text)
        .unwrap();
    serialized_pk
        .add_value(&vec![1_u8, 2_u8], &ColumnType::Blob)
        .unwrap();
    serialized_pk
        .add_value(&scylla::frame::value::CqlTimestamp(1640995200000), &ColumnType::Timestamp)
        .unwrap();

    let token = calculate_token_for_partition_key(&serialized_pk, &partitioner).unwrap();
    println!("token: {}", token.value());
}
