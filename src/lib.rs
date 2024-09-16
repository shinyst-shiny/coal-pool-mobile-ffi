use std::time::Instant;

use serde::{Serialize, Deserialize};

uniffi::include_scaffolding!("drillxmobile");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DxSolution {
    nonce: Vec<u8>,
    digest: Vec<u8>,
    nonces_checked: u32,
    difficulty: u32
}

pub fn dx_hash(challenge: Vec<u8>, cutoff: u64, start_nonce: u64, end_nonce: u64) -> DxSolution {
    let hash_timer = Instant::now();

    let mut challenge_bytes = [0; 32];
    challenge_bytes[0..32].copy_from_slice(&challenge);
    
    let challenge: [u8; 32] = challenge_bytes;
    let mut memory = drillx::equix::SolverMemory::new();
    let mut nonce = start_nonce;
    let mut best_nonce = nonce;
    let mut best_difficulty = 0;
    let mut best_hash = drillx::Hash::default();
    let mut total_hashes: u64 = 0;

    loop {
        // Create hash
        for hx in drillx::hashes_with_memory(
            &mut memory,
            &challenge,
            &nonce.to_le_bytes(),
        ) {
            total_hashes += 1;
            let difficulty = hx.difficulty();
            if difficulty.gt(&7) && difficulty.gt(&best_difficulty) {
                best_nonce = nonce;
                best_difficulty = difficulty;
                best_hash = hx;
            }
        }

        // Exit if processed nonce range
        if nonce >= end_nonce {
            break;
        }

        if hash_timer.elapsed().as_secs().ge(&cutoff) {
            if best_difficulty.ge(&8) {
                break;
            }
        }

        // Increment nonce
        nonce += 1;
    }


    DxSolution {
        nonce: best_nonce.to_le_bytes().to_vec(),
        digest: best_hash.d.to_vec(),
        nonces_checked: total_hashes as u32,
        difficulty: best_difficulty,
    }
}

