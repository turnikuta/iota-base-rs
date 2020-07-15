
use std::env;
use std::process::exit;
use anyhow::Result;
use iota::{
        client::Transfer,
        bundle::TransactionField, 
};
use iota_conversion::Trinary;
use iota_base_rs::{generate_named_seed, 
                   prepare_iota_seed, 
                   get_transaction_hash};

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<_> = env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("Usage: transfer_value  <sender-name> <value> <receiver-name>");
        eprintln!("\nExample: transfer_value Alice 20 Bob");
        exit(1);
    }
    let sender = args[1].to_string();
    let value = args[2].parse::<u64>().unwrap();
    let receiver = args[3].to_string();

    // network to use
    let provider = "https://nodes.comnet.thetangle.org";
    let mwm: u8 = 10;   // comnet

    iota::Client::add_node(&provider)?;

    // generate 'easy to remember' seed from name
    let sender_seed = generate_named_seed(sender);
    let sender_iota_seed = prepare_iota_seed(&sender_seed);

    let receiver_seed = generate_named_seed(receiver);
    let receiver_iota_seed = prepare_iota_seed(&receiver_seed);

    // get receiver's first unused address
    let security: u8 = 2;
    let index: u64 = 0;
    let receiver_address = iota::Client::get_new_address(&receiver_iota_seed)
                                        .security(security)
                                        .index(index)
                                        .generate()
                                        .await?
                                        .1;

    // create a transfer object
    let mut transfers = Vec::new();
    transfers.push(Transfer {
        address: receiver_address,
        value: value,
        message: None,
        tag: None,
    });

    let response = iota::Client::send_transfers(Some(&sender_iota_seed))
                                 .transfers(transfers)
                                 .min_weight_magnitude(mwm)
                                 .send()
                                 .await?;

    println!("Bundle: {:?}", response[0].bundle().to_inner().as_i8_slice().trytes().unwrap());

    for i in 0..response.len() {
        println!("Transaction {} - Index {}", get_transaction_hash(&response[i]), i );
    }

    Ok(())
}
