use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Signer};
use solana_sdk::signer::keypair::read_keypair_file;

fn main() {

    let kris = read_keypair_file("/Users/chris/.config/solana/id.json");
    let krispubkey = kris.unwrap().pubkey();

    let rpc_url = String::from("https://api.devnet.solana.com");
    
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    match client.request_airdrop(&krispubkey, LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };
}