use solana_sdk::signature::{Signer};
use solana_sdk::signer::keypair::read_keypair_file;

fn main() {

    let kriss = read_keypair_file("/Users/chris/.config/solana/id.json");
    println!("Kirss Publickey : {:?}",kriss.unwrap().pubkey());

    let chris = read_keypair_file("/Users/chris/.config/solana/ag.json");
    println!("Chris Publickey : {:?}",chris.unwrap().pubkey());
    
}