
use std::env;
use std::process::exit;
use anyhow::Result;
use iota::bundle::TransactionField;
use iota_conversion::{Trinary, trytes_converter};
use iota_base_rs::get_hash_trits;

#[tokio::main]
async fn main() -> Result<()> {

    // get transaction hash of tail transaction
    let trx_string: String = env::args().skip(1).take(1).collect();
    if trx_string.is_empty() {
        eprintln!("Usage: get_message_from_trx <tail-transaction-hash>");
        eprintln!("\nPlease provide a tail transaction hash (bundle index 0)");
        exit(1);
    }

    // network to use
    let provider = "https://nodes.comnet.thetangle.org";
    iota::Client::add_node(&provider)?;

    let trx_hash = get_hash_trits(&trx_string);

    let transactions = iota::Client::get_bundle(&trx_hash).await?; // Vec<Transaction>

    let trytes_coll: Vec::<String> = transactions.iter()
                                                 // Payload(TritBuf<iota_ternary_preview::t1b1::T1B1Buf>
                                                 .map(|t| t.payload()
                                                          // Trytes
                                                          .to_inner().as_i8_slice().trytes().unwrap()
                                                          // Trytes without traling '9'
                                                          .trim_end_matches('9').to_string())
                                                 .collect();

    let message = match trytes_converter::to_string(&trytes_coll.concat()) {
        Ok(m)  => m,
        Err(e) => { println!("Error: trytes_converter.to_string()\n\t{}", e);
                    exit(1); },
    };
    println!("Message:\n{}", message);

    Ok(())
}

