
use std::env;
use std::process::exit;
use anyhow::Result;
use iota_base_rs::{generate_named_seed, prepare_iota_seed, get_address_trytes};

#[tokio::main]
async fn main() -> Result<()> {

    let name: String = env::args().skip(1).take(1).collect();
    if name.is_empty() {
        eprintln!("Usage: create_account <name-for-seed>");
        eprintln!("\nPlease provide a string (name)!\nThe string is used to generate an 'easy to remember' seed");
        exit(1);
    }

    let provider = "https://nodes.comnet.thetangle.org";
    iota::Client::add_node(&provider)?;

    // generate 'easy to remember' seed from name
    // (but, never use such a seed with real values!)
    let seed = generate_named_seed(name);
    let iota_seed = prepare_iota_seed(&seed);
    println!("SEED: {}", seed);

    // get first unused address
    let security: u8 = 2;  // Security factor 1-3, 2 is used by trinity
    let index: u64 = 0;    // first address
    let response = iota::Client::get_new_address(&iota_seed)
                                 .security(security)
                                 .index(index)
                                 .generate().await?;

    // print index of first unused address and address in legible format
    let (index, address) = response;
    println!("Index: {}", index);
    println!("Address: {:?}", get_address_trytes(&address));

    Ok(())

}

