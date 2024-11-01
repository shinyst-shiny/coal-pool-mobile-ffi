use std::{time::Instant, str::FromStr};

use base64::{prelude::BASE64_STANDARD, Engine};
use bip39::{Mnemonic, Seed};
use serde::{Serialize, Deserialize};
use solana_sdk::{derivation_path::DerivationPath, signature::Keypair, signer::SeedDerivable, system_instruction, pubkey::Pubkey, transaction::Transaction};

uniffi::include_scaffolding!("coalpoolmobileffi");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DxSolution {
    nonce: Vec<u8>,
    digest: Vec<u8>,
    nonces_checked: u32,
    difficulty: u32
}

#[derive(Debug, thiserror::Error)]
pub enum CoalPoolMobileFfiError {
    #[error("Failed to parse pubkey_str {pubkey_str}")]
    InvalidPubkeyStr { pubkey_str: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneratedKey {
    word_list: String,
    keypair: Vec<u8>,
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
            if difficulty.gt(&best_difficulty) {
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
            break;
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

 
pub fn generate_key() -> GeneratedKey {
    let new_mnemonic = Mnemonic::new(bip39::MnemonicType::Words12, bip39::Language::English);
    let phrase = new_mnemonic.clone().into_phrase();

    let seed = Seed::new(&new_mnemonic, "");

    let derivation_path = DerivationPath::from_absolute_path_str("m/44'/501'/0'/0'").unwrap();

    if let Ok(new_key) = Keypair::from_seed_and_derivation_path(seed.as_bytes(), Some(derivation_path)) {
        GeneratedKey {
            word_list: phrase,
            keypair: new_key.to_bytes().to_vec()
        }
    } else {

        GeneratedKey {
            word_list: "failed".to_string(),
            keypair: vec![0u8]

        }
    }
}

pub fn get_transfer_lamports_transaction(latest_blockhash_str: String, from_pubkey_str: String, to_pubkey_str: String, amount: u64) -> Result<String, CoalPoolMobileFfiError> {
    let decoded_blockhash = BASE64_STANDARD.decode(latest_blockhash_str).unwrap();
    let deserialized_blockhash = bincode::deserialize(&decoded_blockhash).unwrap();


    match Pubkey::from_str(&from_pubkey_str) {
        Ok(from_pubkey) => {
            match Pubkey::from_str(&to_pubkey_str) {
                Ok(to_pubkey) => {
                    let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, amount);

                    let mut tx = Transaction::new_with_payer(&[ix], Some(&from_pubkey));

                    tx.message.recent_blockhash = deserialized_blockhash;

                    let serialized_tx = bincode::serialize(&tx).unwrap();

                    let encoded_tx = BASE64_STANDARD.encode(&serialized_tx);

                    Ok(encoded_tx)
                },
                Err(_e) => {
                    return Err(CoalPoolMobileFfiError::InvalidPubkeyStr { pubkey_str: to_pubkey_str })
                }
            }
        },
        Err(_e) => {
            return Err(CoalPoolMobileFfiError::InvalidPubkeyStr { pubkey_str: from_pubkey_str })
        }
    }

}
