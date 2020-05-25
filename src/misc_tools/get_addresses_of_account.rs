
use std::env;
use std::process::exit;
use iota_base_rs::{generate_named_seed, 
                   get_address, 
                   get_address_trytes};

fn main() {

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 4 {
        eprintln!("Usage: get_address_range_of_account <name> <start-index> <count>");
        eprintln!("\nExample: get_address_range_of_account Alice 6 10");
        eprintln!("         ... 10 adresses starting from index 6");
        exit(1);
    }
    let name        = arguments[1].to_string();
    let start_index = arguments[2].parse::<u64>().unwrap();
    let end_index   = start_index + arguments[3].parse::<u64>().unwrap();

    // generate 'easy to remember' seed from name
    // (but, never use such a seed with real values!)
    let seed = generate_named_seed(name);
    println!("SEED: {}", seed);

    for i in start_index..end_index {
        let addr = get_address(&seed, i);
        println!("{:02} : {:?}", i, get_address_trytes(&addr));
    }
}

