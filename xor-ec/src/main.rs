fn main() {
    let input = "Hello, this is a test string!";

    // Pad the string to make its length divisible by 4
    let padded = pad_string(input);

    // Split into 4 equal blocks
    let blocks = split_into_blocks(&padded);

    // Calculate erasure coding block (XOR of all blocks)
    let ec_block = calculate_ec_block(&blocks);

    println!("Original blocks:");
    for (i, block) in blocks.iter().enumerate() {
        println!("Block {}: {}", i + 1, block);
    }
    println!("EC block: {}", ec_block);

    // Simulate loss of block 4 and reconstruct it
    let reconstructed = reconstruct_block(&blocks[0..3], &ec_block);
    println!("\nReconstructed block 4: {}", reconstructed);

    // Verify reconstruction
    assert_eq!(blocks[3], reconstructed);

    // Combine blocks to get original string
    let mut final_string = String::new();
    for i in 0..3 {
        final_string.push_str(&blocks[i]);
    }
    final_string.push_str(&reconstructed);

    // Remove padding
    let final_string = final_string.trim_end_matches('\0');
    println!("\nFinal reconstructed string: {}", final_string);
}

fn pad_string(input: &str) -> String {
    let padding_needed = (4 - (input.len() % 4)) % 4;
    let mut padded = input.to_string();
    padded.extend(std::iter::repeat('\0').take(padding_needed));
    padded
}

fn split_into_blocks(input: &str) -> Vec<String> {
    let block_size = input.len() / 4;
    input
        .chars()
        .collect::<Vec<char>>()
        .chunks(block_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

fn calculate_ec_block(blocks: &[String]) -> String {
    let block_size = blocks[0].len();
    let mut ec_block = vec![0u8; block_size];

    for block in blocks {
        for (i, c) in block.bytes().enumerate() {
            ec_block[i] ^= c;
        }
    }

    String::from_utf8(ec_block).unwrap()
}

fn reconstruct_block(blocks: &[String], ec_block: &str) -> String {
    let block_size = blocks[0].len();
    let mut reconstructed = vec![0u8; block_size];

    // XOR all available blocks and EC block
    for block in blocks {
        for (i, c) in block.bytes().enumerate() {
            reconstructed[i] ^= c;
        }
    }

    // XOR with EC block to get the missing block
    for (i, c) in ec_block.bytes().enumerate() {
        reconstructed[i] ^= c;
    }

    String::from_utf8(reconstructed).unwrap()
}
