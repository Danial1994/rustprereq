use solana_client::rpc_client::RpcClient; use solana_program::{pubkey::Pubkey,system_instruction::transfer,};
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction
    };
use std::str::FromStr;
use solana_program::system_program;
const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod programs;
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};

mod tests {
    use super::*;
    #[test]
    fn submit(){
    
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Let's define our accounts
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq",
        signer.pubkey().to_bytes().as_ref()]);
        // Define our instruction data 
        let args = CompleteArgs {
        github: b"Danial1994".to_vec() };
        // Get recent blockhash
        let blockhash = rpc_client 
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

        // Now we can invoke the "complete" function 
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()], &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash );
        // Send the transaction
        let signature = rpc_client 
        .send_and_confirm_transaction(&transaction) 
        .expect("Failed to send transaction");
        // Print our transaction out
        println!("Success! Check out your TX here:
            https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        }

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();

        // Print the public key (Solana address)
        println!(
            "You've generated a new Solana wallet address: {}",
            kp.pubkey().to_string()
        );

        // Print the private key (in bytes)
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // 1. Read keypair (private key) from the saved JSON wallet file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // 2. Create a connection to the Solana Devnet
        let client = RpcClient::new(RPC_URL);

        // 3. Request an airdrop of 2 SOL (2,000,000,000 lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Airdrop successful! Check your transaction here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    signature.to_string()
                );
            }
            Err(e) => println!("Airdrop failed: {}", e.to_string()),
        }
    } 

    #[test]
    fn transfer_sol() {
            // Import our keypair
            let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
            // Define our Turbin3 public key
            let to_pubkey = Pubkey::from_str("4myPUdieQT3CKs5Vz6u9Z7kgpxj6tmAkpYxh8LEe3eWE").unwrap();
             // Create a Solana devnet connection
             let rpc_client = RpcClient::new(RPC_URL);
             // Get recent blockhash
            let recent_blockhash = rpc_client .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
            let transaction = Transaction::new_signed_with_payer( &[transfer(
                &keypair.pubkey(), &to_pubkey, 1_000_000
                )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
                );
                // Send the transaction
                let signature = rpc_client
                .send_and_confirm_transaction(&transaction)
                .expect("Failed to send transaction");
            
                // Print our transaction out 
                println!(
                "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
                );
    }
    #[test]
    fn empty_wallet() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("4myPUdieQT3CKs5Vz6u9Z7kgpxj6tmAkpYxh8LEe3eWE").unwrap();
         // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Get balance of dev wallet
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        // Get recent blockhash
        let recent_blockhash = rpc_client .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance,
            )], Some(&keypair.pubkey()), &recent_blockhash
            );
        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees 
        let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
        // Deduct fee from lamports amount and create a TX with correct balance 
        let transaction = Transaction::new_signed_with_payer(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,
            )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
        // Send the transaction
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
        
        // Print our transaction out 
        println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
        );

    }

}
