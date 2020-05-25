
use std::env;
use std::process::exit;
use anyhow::Result;
//use serde_json::json;
use iota_base_rs::{get_hash_trits,
                   get_transaction_trytes};

#[tokio::main]
async fn main() -> Result<()> {

    let trx_string: String = env::args().skip(1).take(1).collect();
    if trx_string.is_empty() {
        eprintln!("Usage: get_transaction_trytes <tail-transaction-hash>");
        eprintln!("\nPlease provide a tail transaction hash (bundle index 0)");
        exit(1);
    }

    let provider = "https://nodes.comnet.thetangle.org";
    iota::Client::add_node(&provider)?;

    let trx_hash = get_hash_trits(&trx_string);

    let transactions = iota::Client::get_bundle(&trx_hash).await?;

    for (idx, trx) in transactions.iter().enumerate() {
        println!("Transaction Trytes at Index {}\n{:?}", idx, get_transaction_trytes(&trx));
    }

    Ok(())
}
