
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

    let topic = "sn";

    let subscription = topic.as_bytes(); 
    subscriber.set_subscribe(&subscription)
              .expect("failed setting subscription");

    loop {
        let message = subscriber.recv_string(0)
                                .expect("failed receiving envelope")
                                .unwrap();

        let message_data = message.split(" ").collect::<Vec<&str>>();

        if message_data[0] == "sn" {
            // Index 1: Index of the milestone that confirmed the transaction
            // Index 2: Transaction hash
            // Index 3: Address
            // Index 4: Trunk transaction hash
            // Index 5: Branch transaction hash
            // Index 6: Bundle hash

            println!("Confirmed Transaction: {}", message_data[2]);
        }
    }
}

