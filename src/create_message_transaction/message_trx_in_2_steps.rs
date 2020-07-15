
use anyhow::Result;
use serde_json::json;
use iota::{
        client::Transfer,
        bundle::TransactionField, 
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
    //let iota_seed = &IotaSeed::<Kerl>::from_buf(prepare_iota_seed(&seed)).unwrap();
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
    let _message = json_object.to_string();
    
    let my_short_message = String::from("message trx in 2 steps");
    let _my_long_message = String::from("message trx in 2 steps ".repeat(100));

    let mut transfers = Vec::new();
    transfers.push(Transfer {
        address: address,
        value: 0,
        message: Some(my_short_message),
        tag: prepare_tag_field("TURNIKUTA"),
    });

    //
    // Step 2 - send transfer 
    //
    //   pepare_transfer()
    //   send_trytes()
    //       get_transactions_to_approve()     
    //       attach_to_tangle()     
    //       store_and_broadcast()
    //

    //
    // send_transfers(None) doesn't work (see https://github.com/iotaledger/iota.rs/issues/100)
    //
    let response = iota::Client::send_transfers(Some(&iota_seed))
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
