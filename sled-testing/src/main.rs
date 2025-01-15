use sled::transaction::ConflictableTransactionError;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let _ = std::fs::remove_dir_all("my_db");
    let db: sled::Db = sled::open("my_db").unwrap();
    println!("DB initialization: {:?}", start.elapsed());

    // insert and get
    let start = Instant::now();
    db.insert(b"yo!", b"v1").unwrap();
    println!("Insert operation: {:?}", start.elapsed());

    // flush the insert
    let start = Instant::now();
    db.flush().unwrap();
    println!("Flush operation: {:?}", start.elapsed());

    let start = Instant::now();
    assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");
    println!("Get operation: {:?}", start.elapsed());

    // Atomic compare-and-swap
    let start = Instant::now();
    let _ = db.compare_and_swap(
        b"yo!",      // key
        Some(b"v1"), // old value, None for not present
        Some(b"v2"), // new value, None for delete
    )
    .unwrap();
    println!("Compare-and-swap operation: {:?}", start.elapsed());

    let start = Instant::now();
    db.flush().unwrap();
    println!("Flush operation: {:?}", start.elapsed());

    // Range iteration
    let start = Instant::now();
    let scan_key: &[u8] = b"a non-present key before yo!";
    let mut iter = db.range(scan_key..);
    assert_eq!(&iter.next().unwrap().unwrap().0, b"yo!");
    assert_eq!(iter.next(), None);
    println!("Range iteration: {:?}", start.elapsed());

    // Remove
    let start = Instant::now();
    db.remove(b"yo!").unwrap();
    assert_eq!(db.get(b"yo!"), Ok(None));
    println!("Remove operation: {:?}", start.elapsed());

    // Open tree
    let start = Instant::now();
    let other_tree: sled::Tree = db.open_tree(b"cool db facts").unwrap();
    other_tree
        .insert(
            b"k1",
            &b"a Db acts like a Tree due to implementing Deref<Target = Tree>"[..],
        )
        .unwrap();
    println!("Open tree and insert: {:?}", start.elapsed());

    let start = Instant::now();
    db.flush().unwrap();
    println!("Flush operation: {:?}", start.elapsed());

    // Transaction
    let start = Instant::now();
    let res = db.transaction(|tx_db| -> Result<Vec<u8>, ConflictableTransactionError> {
        tx_db.insert(b"k1", b"cats")?;
        tx_db.insert(b"k2", b"dogs")?;
        let v = tx_db.get(b"k1")?;
        Ok(v.unwrap().to_vec())
    })
    .unwrap();
    db.flush().unwrap();

    println!("Transaction operation with flush: {:?}", start.elapsed());
    println!("v: {:?}", String::from_utf8(res));

    // Cleanup
    let start = Instant::now();
    std::fs::remove_dir_all("my_db").unwrap();
    println!("Database cleanup: {:?}", start.elapsed());
}
