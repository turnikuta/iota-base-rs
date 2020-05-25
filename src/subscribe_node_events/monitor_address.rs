
use std::env;
use std::process::exit;
use zmq;

fn main() {

    let addr: String = env::args().skip(1).take(1).collect();
    if addr.is_empty() || addr.len() <81 {
        eprintln!("Address missing or wrong...");
        eprintln!("\n\tUsage: monitor_address <81-Tryte-Address>\n");
        exit(1);
    }


    //
    // --> Please add your comnet node >>>here<<< 
    //     The ZMQ Plugin musst be enabled!
    //
    let sock_provider = "tcp://<your-zmq-comnet-node>:5556";
    println!("ZMQ Node: {}", sock_provider);

    let ctx = zmq::Context::new();
    let subscriber = ctx.socket(zmq::SUB).unwrap();
    subscriber.connect(sock_provider)
              .expect("failed connecting subscriber");

    let topic = addr;

    let subscription = topic.as_bytes(); 
    subscriber.set_subscribe(&subscription)
              .expect("failed setting subscription");

    loop {
        let message = subscriber.recv_string(0)
                                .expect("failed receiving envelope")
                                .unwrap();
        let message_data = message.split(" ").collect::<Vec<&str>>();

        // Index 1: Address (you are monitoring) - same as Index 0
        // Index 2: Transaction hash of a confirmed transaction that the address appeared in
        // Index 3: Milestone that confirmed the transaction 
        println!("Transaction: {}", message_data[1]);
    }
}

