use serde::{Deserialize, Serialize};
use std::io::Cursor;

/**
 * Here you will set the types of request that will interact with the raft nodes.
 * For example the `Set` will be used to write data (key and value) to the raft database.
 * The `AddNode` will append a new node to the current existing shared list of nodes.
 * You will want to add any request that can write data in all nodes here.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Set { key: String, value: String },
}

/**
 * Here you will defined what type of answer you expect from reading the data of a node.
 * In this example it will return a optional value from a given key in
 * the `Request.Set`.
 *
 * TODO: Should we explain how to create multiple `AppDataResponse`?
 *
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub value: Option<String>,
}

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
// pub struct Node {
//     pub rpc_addr: String,
//     pub api_addr: String,
// }

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
);

fn main() {
    println!("Hello, world!");
}
