
use std::env;
use std::process::exit;
use anyhow::Result;
use iota_base_rs::{generate_named_seed, 
                   prepare_iota_seed, 
                   get_address_trytes};

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<_> = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: get_account_info  <name-for-seed>");
        eprintln!("\nPlease provide the string (name) that was used to generate an 'easy to remember' seed");

        exit(1);
    }
    let name = args[1].to_string();

    let provider = "https://nodes.comnet.thetangle.org";
    iota::Client::add_node(&provider)?;

    // generate 'easy to remember' seed from name
    // (but, never use such a seed with real values!)
    let seed = generate_named_seed(name);
    let iota_seed = prepare_iota_seed(&seed);

    // get the first unused address
    let security: u8 = 2;
    let start_index: u64 = 0;
    let (index, addr) = iota::Client::get_new_address(&iota_seed)
                                      .security(security)
                                      .index(start_index)
                                      .generate()
                                      .await?;

    // get balance from address 
    let response = iota::Client::get_balances()
                                 .addresses(&[addr.clone()])
                                 .send()
                                 .await?;

    // print account info: <index address balance>
    println!("{} {:?} {}", index, get_address_trytes(&addr.clone()), response.balances[0]);
    Ok(())

}

