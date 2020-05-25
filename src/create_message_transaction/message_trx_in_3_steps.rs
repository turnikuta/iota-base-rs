
use anyhow::Result;
use serde_json::json;
use iota::{
        client::Transfer,
        bundle::{Transaction, TransactionField}, 
};
use iota_conversion::Trinary;
use iota_base_rs::{generate_random_seed, 
                   prepare_iota_seed, 
                   get_transaction_hash,
                   prepare_tag_field};

#[tokio::main]
async fn main() -> Result<()> {

    // network to use
    let provider = "https://nodes.comnet.thetangle.org";
    let mwm: u8 = 10;    // comnet

    iota::Client::add_node(&provider)?;

    // generate 'easy to remember' seed from name
    let seed = generate_random_seed();
    let iota_seed = prepare_iota_seed(&seed);

    // get address
    let security: u8 = 2;
    let index: u64 = 0;
    let address = iota::Client::get_new_address(&iota_seed)
                                .security(security)
                                .index(index)
                                .generate()
                                .await?
                                .1;
    
    //
    // Step 1 - create a transfer object
    //
    let json_object = json!({
        "name": "document.odt",
        "version": "2.4",
        "size": 306598,
        "modified": "2020-05-10T05:11:20.000Z",
    });
    let message = json_object.to_string();
    
    let _my_short_message = String::from("message trx in 3 steps");
    let _my_long_message = String::from("message trx in 3 steps ".repeat(100));

    let mut transfers = Vec::new();
    transfers.push(Transfer {
        address: address,
        value: 0,
        message: Some(message),
        tag: prepare_tag_field("TURNIKUTA"),
    });

    //
    // Step 2 - prepare transfer 
    //
    let bundle = iota::Client::prepare_transfers(&iota_seed)
                               .transfers(transfers)
                               .security(security)
                               .build()
                               .await?;
   
    let trytes: Vec<Transaction> = bundle.into_iter().map(|x| x).collect();

    //
    // Step 3 - send trytes
    //       get_transactions_to_approve()
    //       attach_to_tangle()
    //       store_and_broadcast()
    //
    let depth: u8 = 3;
    let response = iota::Client::send_trytes()
                                 .trytes(trytes)
                                 .min_weight_magnitude(mwm)
                                 .depth(depth)
                                 .send()
                                 .await?;

    println!("Bundle: {:?}", response[0].bundle().to_inner().as_i8_slice().trytes().unwrap());

    for i in 0..response.len() {
        println!("Transaction {} - Index {}", get_transaction_hash(&response[i]), i );
    }

    Ok(())
}
