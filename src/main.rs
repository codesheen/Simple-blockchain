
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    let difficulty = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"; // Задаём сложность майнинга.
    let mut nonce: u64 = 0; // nonce — число, которое изменяется для поиска подходящего хеша.
    let mut index: u64 = 0; // индекс блока, который мы майним.
    let mut previous_hash = [1u8; 32];
    
    let mut balance: f64= 0.0;
    let mut reward: f64 = 17.592169267200; 


    #[derive(Debug, Clone)]
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

        hasher.update(timestamp.to_le_bytes()); // Добавляем в хешер текущее время.
        hasher.update(nonce.to_le_bytes()); // Добавляем в хешер nonce.
        hasher.update(index.to_le_bytes()); // Добавляем в хешер индекс блока.
        hasher.update(previous_hash); // Добавляем в хешер предыдущий хеш.

        let hash = hasher.finalize();
        let double_hash = Sha256::digest(&hash);
        let hash_hex = format!("{:x}", double_hash);
        let previous_hash_hex: String = previous_hash.iter().map(|byte| format!("{:02x}", byte)).collect();

        if hash_hex.as_str() <= difficulty {
            // println!("\n**********************");
            // println!("*****BLOCK {} MINED!*****", index);
            // println!("**********************");
            // println!("Timestamp: {}", timestamp);
            // println!("previous_hash: {}", previous_hash_hex);
            // println!("Nonce: {}", nonce);
            // println!("Hash: {}", hash_hex);
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

            if blockchain.len() > 1 {
                let current = blockchain.last().unwrap();
                let previous = &blockchain[blockchain.len() - 2];

                if current.previous_hash == previous.hash {
                    //println!("Block VALID!");
                } else {
                    //println!("Block INVALID!");
                }
            }
            
            previous_hash.copy_from_slice(&double_hash);

            nonce = 0;
            index += 1;
            println!("Total blocks: {}", blockchain.len());


            println!("{}", reward);

            reward = (reward / 1.00000153549216).max(0.6);
        }

        nonce += 1;
        println!("\rNonce: {}", nonce);
        if reward == 0.6 {
            println!("Reward has reached the minimum value of 0.6 BTC.");

        }
    }
}
