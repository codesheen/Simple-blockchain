use sha2::{Sha256, Digest};
use std::io::{self, Chain, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use serde::{Serialize, Deserialize};

fn main() {
    const ATOMS_PER_COIN: u128 = 1_000_000_000_000;

    let difficulty_hex = "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let mut nonce: u64 = 0;
    let mut index: u64 = 0;
    let mut previous_hash = [1u8; 32];
    
    let mut balance: u128 = 0;
    let mut reward: u128 = 17_592_169_267_200;

    let mut difficulty = [0u8; 32];

    hex::decode_to_slice(difficulty_hex, &mut difficulty).unwrap();
        

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[allow(dead_code)]
    struct Block {
        timestamp: u64,
        nonce: u64,
        index: u64,
        previous_hash: [u8; 32],
        hash: [u8; 32],
    }

    let mut blockchain: Vec<Block> = Vec::new();

    loop {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut hasher = Sha256::new();

        hasher.update(timestamp.to_le_bytes());
        hasher.update(nonce.to_le_bytes());
        hasher.update(index.to_le_bytes());
        hasher.update(previous_hash);

        let hash = hasher.finalize();
        let double_hash = Sha256::digest(&hash);

        let block_hash: [u8; 32] = double_hash.into();

        //let hash_hex = format!("{:x}", double_hash);
        let previous_hash_hex: String = previous_hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        if block_hash <= difficulty {
            println!("\n**********************");
            println!("*****BLOCK {} MINED!*****", index);
            println!("**********************");
            println!("Timestamp: {}", timestamp);
            println!("previous_hash: {}", previous_hash_hex);
            println!("Nonce: {}", nonce);
            //println!("Hash: {}", block_hash);
            balance += reward;
            println!("Balance: {} BTC", balance);
            
            let mut block_hash = [0u8; 32];
            block_hash.copy_from_slice(&double_hash);

            let block = Block {
                timestamp,
                nonce,
                index,
                previous_hash,
                hash: block_hash,
            };

            blockchain.push(block);

            let mut chain = serde_json::to_string_pretty(&blockchain).unwrap();
            fs::write("blockchain.json", chain).expect("Unable to write file");

            if blockchain.len() > 1 {
                let current = blockchain.last().unwrap();
                let previous = &blockchain[blockchain.len() - 2];

                if current.previous_hash == previous.hash {
                    println!("Block VALID!");
                } else {
                    println!("Block INVALID!");
                }
            }
            
            previous_hash.copy_from_slice(&double_hash);

            nonce = 0;
            index += 1;
            println!("Total blocks: {}", blockchain.len());

            println!("Reward: {}", reward);

            const MIN_REWARD: u128 = 600_000_000_000;

            let atoms: u128 = balance;
            const ATOMS_PER_COIN: u128 = 1_000_000_000_000;

            println!(
                "{}.{:012}", atoms / ATOMS_PER_COIN, atoms % ATOMS_PER_COIN);

            reward = (reward * 100_000_000_000_000 / 100_090_153_549_216).max(MIN_REWARD);
        }

        nonce += 1;
    }
}