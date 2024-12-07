use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use uuid::Uuid;

// Constants for the test
const TOTAL_DATA_POINTS: usize = 100_000_000; // Adjust based on available memory
const QUERIES: usize = 1_000_000;
const NUM_PARTITIONS: usize = 100;

fn generate_test_data(size: usize) -> Vec<(Uuid, String)> {
    println!("Generating {} random UUIDs...", size);
    (0..size)
        .map(|i| (Uuid::new_v4(), format!("value_{}", i)))
        .collect()
}

fn test_single_btreemap(data: &[(Uuid, String)]) {
    println!("\nTesting single large BTreeMap...");

    // Create and populate single tree
    let single_tree = Arc::new(Mutex::new(BTreeMap::new()));
    for (key, value) in data.iter() {
        single_tree.lock().unwrap().insert(*key, value.clone());
    }

    // Perform random queries
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    for _ in 0..QUERIES {
        let random_key = &data[rng.gen_range(0..data.len())].0;
        let _ = single_tree.lock().unwrap().get(random_key);
    }
    println!("Single tree query time: {:?}", start.elapsed());
}

fn test_partitioned_btreemap(data: &[(Uuid, String)]) {
    println!("\nTesting partitioned BTreeMaps...");

    // Create and initialize partitioned maps
    let partitioned_maps: Arc<Mutex<HashMap<u8, Arc<Mutex<BTreeMap<Uuid, String>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    {
        let mut maps = partitioned_maps.lock().unwrap();
        for i in 0..NUM_PARTITIONS {
            maps.insert(i as u8, Arc::new(Mutex::new(BTreeMap::new())));
        }
    }

    // Populate partitioned maps
    for (key, value) in data.iter() {
        let partition = (key.as_bytes()[0] as usize) % NUM_PARTITIONS;
        let maps = partitioned_maps.lock().unwrap();
        if let Some(tree) = maps.get(&(partition as u8)) {
            tree.lock().unwrap().insert(*key, value.clone());
        }
    }

    // Perform random queries

    // How data is partitioned and rebalanced isn't as relevant to the lookup performance
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    for _ in 0..QUERIES {
        let random_key = &data[rng.gen_range(0..data.len())].0;
        let partition = (random_key.as_bytes()[0] as usize) % NUM_PARTITIONS;
        let maps = partitioned_maps.lock().unwrap();
        if let Some(tree) = maps.get(&(partition as u8)) {
            let _ = tree.lock().unwrap().get(random_key);
        }
    }
    println!("Partitioned maps query time: {:?}", start.elapsed());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <test-type>", args[0]);
        println!("  test-type options:");
        println!("    single    - Test single large BTreeMap");
        println!("    partition - Test partitioned BTreeMaps");
        println!("    all      - Run all tests");
        return;
    }

    let test_data = generate_test_data(TOTAL_DATA_POINTS);

    match args[1].as_str() {
        "single" => test_single_btreemap(&test_data),
        "partition" => test_partitioned_btreemap(&test_data),
        "all" => {
            test_single_btreemap(&test_data);
            test_partitioned_btreemap(&test_data);
        }
        _ => println!("Invalid test type. Use 'single', 'partition', or 'all'"),
    }
}
