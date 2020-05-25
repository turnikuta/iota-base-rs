This little demo shows how to monitor events published by an IOTA node

The demo contains the following scripts:
- **monitor_address.rs** 
- **monitor_confirmed_trx.rs** 
- **monitor_value_trx.rs** 

#### 1.  Monitor a given address for confirmed transactions

**Example:** 
```
$ cargo run  --bin monitor_address MGDV9XKLUHPXRPOBULDSIHIYUZRCRMZUVXNFOPVDVVJEPWAIGLSJXLBZGMFLVPNZQIVDUYCGTZLGOMJCY  
    Finished dev [unoptimized + debuginfo] target(s) in 2.60s
     Running `target/debug/monitor_address MGDV9XKLUHPXRPOBULDSIHIYUZRCRMZUVXNFOPVDVVJEPWAIGLSJXLBZGMFLVPNZQIVDUYCGTZLGOMJCY`
Transaction: WLJF9ZGXXNOYNHRDF9YFPAWHDHGKYRFVKI9VEUHMWWKQYRZYOQMGVA9VVCJ9ZPSRGQQXEPMWDUONXW999
```

#### 2. Monitor confirmed transactions

**Example:** 
```
$ cargo run --bin monitor_confirmed_trx
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/monitor_confirmed_trx`
Confirmed Transaction: Y9UGYLPOLOLRMKPFLTUAYOZWDGXLHPHZLNEGKQFDJM9ZLTNFEXJM9ICIW9DSYEISKNNAXGZCERAEWD999
Confirmed Transaction: EFXMMXGXTNCOCZSLLPAHDRGBDTRACIOXNYC9MPBYKWIVXISCFNDUIIMJQRLRFFWXBRU9ZPPFCJ9A9Y999
Confirmed Transaction: FCKOSTD9DYOGJTARHHYOYYGPDLOIVFWBVUT99WKDVGUMRONYYFYJIWLHL9LGNVS9MDWNNKR9DNGKSZ999
Confirmed Transaction: NZSWBIRBDYYVWIEYNYEQFIIFVVXGQTTJAT9JYEWXTZKOMJMAENYELLHGRLIBDQXFXKTRXPQCJDCONB999
Confirmed Transaction: JNQOMLZIKEIUVRJLPIUKETWMPNYMJD9SVIPBWYVSASWW9HIYKHONQUS9DMYZIXM9YU9YJCQLZTNYQ9999
Confirmed Transaction: UM9IIB99BEF9BGZHTT9XUWUWRTRDCEHEZPGSRUOISIFT9UD99IEIQSPFYOHSBBMAPMIDZLNQULL99C999
Confirmed Transaction: NDDXWH9MFSAKDGPJHXZHIKQPKYANKKFELAYHBFBEZERAUFBOOKEZUMSCAL9SJDSFEVDPSSRJYUN9L9999
```

#### 3. Monitor value transactions

**Example:** 
```
$ cargo run --bin monitor_alue_trx
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/monitor_value_trx`
Value Transaction: I9PWQCPWKU9XUBUFYIUMONAXYBETXHIHETKJGXVGFHAWL9KUMLILMXAQMGZFUCAIIZXGQXYNSWIRTB999, Value: 50
Value Transaction: OTEMXHPGWVLULS9DVLJYTJBQOHDN9ODWWANVVFFSMVJHCOKPEMWMKJIZVDYPLBJOCIGHQSYMLTFUPA999, Value: 900

```
