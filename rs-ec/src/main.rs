use reed_solomon_erasure::galois_8::{ReedSolomon, ShardByShard};
// or use the following for Galois 2^16 backend
// use reed_solomon_erasure::galois_16::ReedSolomon;
use std::time::Instant;
use rand::Rng;

fn main() {
    println!("Shard by shard reconstruction:");
    shard_by_shard();
    println!("\nAt once reconstruction:");
    at_once();
    println!("\nLarge data reconstruction:");
    large_data();
}

fn shard_by_shard () {
    // Encode with shard by shard
    let r = ReedSolomon::new(3, 2).unwrap(); // 3 data shards, 2 parity shards

    let mut sbs = ShardByShard::new(&r);

    let mut original_data: Vec<Vec<u8>> = vec!(
        vec![0, 1,  2,  3],
        vec![4, 5,  6,  7], // pretend we don't have this shard
        vec![0, 0,  0,  0], // pretend we don't have this shard
        vec![0, 0,  0,  0], // last 2 rows are parity shards
        vec![0, 0,  0,  0]
    );

    // Encode the first two data shards
    sbs.encode(&mut original_data).unwrap();
    sbs.encode(&mut original_data).unwrap();

    // Add the third data shard
    original_data[2] = vec![8, 9, 10, 11];

    // Encode the third data shard
    sbs.encode(&mut original_data).unwrap();


    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = original_data.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    shards[0] = None;
    shards[4] = None;

    let timer = Instant::now();
    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();
    let elapsed = timer.elapsed();
    println!("Reconstruction time: {:?}", elapsed);

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    assert!(r.verify(&result).unwrap());
    assert_eq!(original_data, result);
}

fn at_once () {
    // Encode with shard by shard
    let r = ReedSolomon::new(3, 2).unwrap(); // 3 data shards, 2 parity shards

    let mut original_data: Vec<Vec<u8>> = vec!(
        vec![0, 1,  2,  3],
        vec![4, 5,  6,  7],
        vec![8, 9, 10, 11],
        vec![0, 0,  0,  0], // last 2 rows are parity shards
        vec![0, 0,  0,  0]
    );

    // Construct the parity shards
    r.encode(&mut original_data).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = original_data.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    shards[0] = None;
    shards[4] = None;

    let timer = Instant::now();
    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();
    let elapsed = timer.elapsed();
    println!("Reconstruction time: {:?}", elapsed);

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    assert!(r.verify(&result).unwrap());
    assert_eq!(original_data, result);
}

fn large_data() {
    // Calculate shard size
    const TOTAL_SIZE: usize = 64 * 1024 * 1024; // 64MB in bytes
    const SHARD_SIZE: usize = TOTAL_SIZE / 4;    // Split across 4 data shards
    
    // Create Reed-Solomon encoder with 4 data shards and 2 parity shards
    let r = ReedSolomon::new(4, 2).unwrap(); // 4 data shards, 2 parity shards

    // Generate random data for each shard
    let mut rng = rand::thread_rng();
    let mut original_data: Vec<Vec<u8>> = vec![
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 1
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 2
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 3
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 4
        vec![0; SHARD_SIZE],                          // Parity shard 1
        vec![0; SHARD_SIZE],                          // Parity shard 2
    ];

    println!("Encoding {} MB of data with 4+2 encoding...", TOTAL_SIZE / 1024 / 1024);
    let encode_timer = Instant::now();
    // Encode to generate parity shards
    r.encode(&mut original_data).unwrap();
    let encode_time = encode_timer.elapsed();
    println!("Encoding time: {:?}", encode_time);

    // Create reconstruction scenario
    let mut shards: Vec<_> = original_data.iter().cloned().map(Some).collect();
    
    // Remove two shards (can remove up to 2 with this configuration)
    shards[0] = None;
    shards[4] = None;

    println!("Reconstructing 2 missing shards...");
    let reconstruct_timer = Instant::now();
    // Reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();
    let reconstruct_time = reconstruct_timer.elapsed();
    println!("Reconstruction time: {:?}", reconstruct_time);

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    // Verify the reconstruction
    assert!(r.verify(&result).unwrap());
    assert_eq!(original_data, result);
    println!("Verification successful!");
}
