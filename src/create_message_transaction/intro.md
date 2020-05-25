## Introduction

The scripts show how to save a message in the IOTA Tangle. The Rust client library iota.rs provides the required functions. The creation and sending of a transaction basically requires 5 steps, whereas the library allows to combine steps.

- message_trx_in_5_steps.rs
- message_trx_in_3_steps.rs
- message_trx_in_2_steps.rs
- get_message_from_trx.rs

#### Create Message Transaction in 5 Steps
**Example 1:** Long Message that fits in 3 transaction
```
$ cargo run --bin message_trx_in_5_steps
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/message_trx_in_5_steps`
Bundle: "WXZQKLNELHAFDPQGJUOV9P9IUHEHZROZKGWKAPZAFGQUHFIOUO9XLCXH9GG9BR9OWKHQIHGDQESBVPNYY"
Transaction DUBYHSWIIJSTSQRAGGSXEXKCZDVTPPOXSVARVXTTPYWLK9MMCUXCXVTXC9AWQKCNNXWSOVOBZXFFEW999 - Index 0
Transaction MROKRTO9MSHOIPZDWTHPCCFZMNLZCRUIVPEAHWSWQAEYCVNCHNDLLXYEBPBJNHRQKTFVMAMMQSR9RY999 - Index 1
Transaction MBWFBVBWLCSJRUODEBIHAFMEFVIPWBCQXXETURMNMSDJZGPSEXVOKAOBRRQTIHPXJINW9DXIONMKLX999 - Index 2
```

#### Create Message Transaction in 2 Steps
**Example 2:** short message that needs 1 transactions
```
$ cargo run --bin message_trx_in_2_steps
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/message_trx_in_2_steps`
Bundle: "WD99TLWWJOEHXJJUJUQX9GH9QYSEIHPQABJDQPQTJVKDLVJAPAFEQ9KIQBCBXRGGGXHEKFTOAJLGKFVXW"
Transaction PMUE9TMVVQMKKPYGQONASWWEZNTSITDZKOXNUTZEHAVJKBMKFGEFDTGF9SWYUWZNVQSLMGIPUTADFC999 - Index 0
```

#### Create Message Transaction in 3 Steps
**Example 3:** short message with JSON document that needs 1 transactions
```
$ cargo run --bin message_trx_in_3_steps
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/message_trx_in_3_steps`
Bundle: "DGIGATBULJQEJO9NICKISEYEI9OUOPQIDIREAELJEFJCLHNELDXJYCFTMWCGUGRIBDRACFBGAZZXEKRAY"
Transaction CTFTRTZFSDKXVREQLZLZKGTSEQGWGECETSAFTIQSLHQNTMHEWRNBYOZDADPYYEST9LZ9LKDI9BWIA9999 - Index 0
```

#### Retrieve Message from the Tangle
**Remark:** Always pass the tail transaction (Index 0) to the script

Retrieve Message from Example 1
```
$ cargo run --bin get_message_from_trx DUBYHSWIIJSTSQRAGGSXEXKCZDVTPPOXSVARVXTTPYWLK9MMCUXCXVTXC9AWQKCNNXWSOVOBZXFFEW999
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_message_from_trx DUBYHSWIIJSTSQRAGGSXEXKCZDVTPPOXSVARVXTTPYWLK9MMCUXCXVTXC9AWQKCNNXWSOVOBZXFFEW999`
Message:
message trx in 5 steps message trx in 5 steps message trx in 5 steps message trx in 5 steps message trx in 5 steps.....
```

Retrieve Message from Example 2
```
$ cargo run --bin get_message_from_trx PMUE9TMVVQMKKPYGQONASWWEZNTSITDZKOXNUTZEHAVJKBMKFGEFDTGF9SWYUWZNVQSLMGIPUTADFC999
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/get_message_from_trx PMUE9TMVVQMKKPYGQONASWWEZNTSITDZKOXNUTZEHAVJKBMKFGEFDTGF9SWYUWZNVQSLMGIPUTADFC999`
Message:
message trx in 2 steps
```

Retrieve Message from Example 3
```
$ cargo run --bin get_message_from_trx CTFTRTZFSDKXVREQLZLZKGTSEQGWGECETSAFTIQSLHQNTMHEWRNBYOZDADPYYEST9LZ9LKDI9BWIA9999
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_message_from_trx CTFTRTZFSDKXVREQLZLZKGTSEQGWGECETSAFTIQSLHQNTMHEWRNBYOZDADPYYEST9LZ9LKDI9BWIA9999`
Message:
{"modified":"2020-05-10T05:11:20.000Z","name":"document.odt","size":306598,"version":"2.4"}
```
