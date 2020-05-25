
use zmq;

fn main() {

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

    let topic = "tx";

    let subscription = topic.as_bytes(); 
    subscriber.set_subscribe(&subscription)
              .expect("failed setting subscription");

    loop {
        let message = subscriber.recv_string(0)
                                .expect("failed receiving envelope")
                                .unwrap();

        let message_data = message.split(" ").collect::<Vec<&str>>();

        if message_data[0] == "tx" {
            // Index 1: Transaction hash
            // Index 2: Address
            // Index 3: Value
            // Index 4: Obsolete tag
            // Index 5: Value of the transaction's timestamp field
            // Index 6: Index of the transaction in the bundle
            // Index 7: Last transaction index of the bundle
            // Index 8: Bundle hash
            // Index 9: Trunk transaction hash
            // Index 10: Branch transaction
            
            let value: i64 = message_data[3].parse().unwrap();
            if value > 0 {
                println!("Value Transaction: {}, Value: {}", message_data[1], value);
            }
        }
    }
}
