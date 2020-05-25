This little demo shows how to create and send a value transaction. 

The demo contains the following scripts:
- **create_account.rs** - generate a 'easy-to-remeber' seed
- **transfer_value.rs** - create and send value transaction
- **get_account_info.rs** - show the account balance 


#### 1. Create Accounts

**Example:** Create the two accounts "Karen" and "Richard"
```
$ cargo run  --bin create_account karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/create_account karen`
SEED: KAREN9999999999999999999999999999999999999999999999999999999999999999999999999999
Index: 0
Address: "YLXSEZIW9FUEBQHWQTJM9RIIEIQIQTEDANCPY9KDOLIYVGUPIGAKRCOECPRGJVFEFKXFOKTNXUCJVA9OZ"

$ cargo run  --bin create_account Richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/create_account Richard`
SEED: RICHARD99999999999999999999999999999999999999999999999999999999999999999999999999
Index: 0
Address: "BOOLNPSJATRTZATOWIJZVJCZSYMMTWVIXKFHRTRNIAAZXDERY9SOIQYDYGKMCLNV9YFYVCNKTTQVMUVUD"
```
**Note**
- the 'easy-to-remember' seed (.... but, never use such a seed with real values!)
- the first unused address (index 0) 


#### 2. Deposit Tokens

Use the IOTA Faucet  https://faucet.comnet.einfachiota.de/  and deposit 1K IOTA tokens on Karen's first address

You can check the status of the transaction on https://comnet.thetangle.org

Once the transaction has been confirmed, you can check the balances.
```
$ cargo run  --bin get_account_info karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/get_account_info karen`
0 "YLXSEZIW9FUEBQHWQTJM9RIIEIQIQTEDANCPY9KDOLIYVGUPIGAKRCOECPRGJVFEFKXFOKTNXUCJVA9OZ" 1000

$ cargo run  --bin get_account_info richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_account_info richard`
0 "BOOLNPSJATRTZATOWIJZVJCZSYMMTWVIXKFHRTRNIAAZXDERY9SOIQYDYGKMCLNV9YFYVCNKTTQVMUVUD" 0
```
#### 3. Transfer Tokens

**Example:** "Karen" sends 100 IOTA to "Richard"
```
$ cargo run  --bin transfer_value karen 100 richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/transfer_value karen 100 richard`
Bundle: "XS9EQQDKOEHCNIKKSJRQSENAC9J9FRZXIAGZBUGVBBCIOV9WWADWNCRQKBFFH9CONZYKJCIVQDSPLEU9Z"
Transaction NHDNR9BGNTT9XHPQEDOFKDUEAYHYWXQZ9NYGNJEP9DZGYHCZAFCJRHBRSLDIMNVEQUMGSSGABVFOOY999 - Index 0
Transaction TPJTIPIYRBOHJRSEPYBWFQOPIRGGHSPMFVHJLWDUFNRFUWRELZES9ZJWFKUBGXVCH9GAXNLASDP9BW999 - Index 1
Transaction OTRMIUHLCOCINHKOKGYJNVMYWOQDHMOHUJKYTTIOAUUGCNGBBUEWVYIULSNTBGTVIVDKZHQIVSOSNA999 - Index 2
Transaction TXSOGZYYVIGJRQZZLPYOXDNWSXRPJEMJBTIGDAOEKMXCCW9MLHW9TUICKEGPCNIARJPHQOYENRVRNC999 - Index 3
```
Withe the default security level 2 a total of 4 transactions were generated. 

Two "Input Transaction" and Two "Output Transactions":
- Index 0, Tail transaction with recipient's address (value = + 100)
- Index 1, Sender's address and its signature + (value = - 1000)
- Index 2, Sender's address and the rest of its signature (value = 0)
- Index 3. Remainder address (value = + 900)

#### 4. Check Balances

Once the transaction has been confirmed, you can check the balances.
```
$ cargo run  --bin get_account_info karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_account_info karen`
1 "ZDPMMULDACOWR9ACIRRREJMHSFSZDLUDIEUHFMMFEFPPJTPRZZVEAIKQXPAHNQTCYAXLBZBGYBCMULIUC" 900

$ cargo run  --bin get_account_info richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_account_info richard`
0 "BOOLNPSJATRTZATOWIJZVJCZSYMMTWVIXKFHRTRNIAAZXDERY9SOIQYDYGKMCLNV9YFYVCNKTTQVMUVUD" 100
```
**Remark**
- Karen's remaining balance is now stored on a different address (Index 1)
- Richard's new balance is stored on the first unused address (Index 0)


#### 5. Miscellaneous 

The transfer script always uses the receivers first unused address. 

**Example:** "Karen" transfers tokens to "Richard" again
```
$ cargo run  --bin transfer_value karen 50 richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/transfer_value karen 50 richard`
Bundle: "JXQBWFTLZCWDBO9FTOR9IUFEFJLTNIGICFLOARKRDXIUVEPWQLLGXCSIHHXRFCQULGAHGPKZUUOBGRUT9"
Transaction NUHXWWNEHHJIBIDMDDQOHCCFIPEKIUJYFNEJISFAOMGCJJNRPUNALHJNQMPLVQDXOQNMOZUB9G9KWZ999 - Index 0
Transaction XFF9DXHZFBKBCQQIITNQZ9HHTZUILSXLWGICBJBHGXFNYTFXWPPNOSQRUBVNXFBTYWFIMYUE9BURQB999 - Index 1
Transaction YIWVPEWP9YDQEIBBNQIPASDPZJHDBBVRWIFFOBQVQPTOBWMZZWIJCCHQRUIL9WEQLWIKTHZFGLWDBW999 - Index 2
Transaction WNMBJTBLSOUCZKFEGZGJSKBAOQGLHAMSEHEGLDIJTOFPYWGCTTOV9RYEGDVNKVQSPIJVYAAMXBZMT9999 - Index 3
```

Richard has not yet sent any tokens, so his first address (Index 0) is still unused
```
$ cargo run  --bin get_account_info richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/get_account_info richard`
0 "BOOLNPSJATRTZATOWIJZVJCZSYMMTWVIXKFHRTRNIAAZXDERY9SOIQYDYGKMCLNV9YFYVCNKTTQVMUVUD" 150
```

Karen's remaining balance is now stored on the third address (Index 2)
```
$ cargo run  --bin get_account_info karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/get_account_info karen`
2 "XPEVRJE9JIYGKXRANSMT9WHFIJMJCW9LXU9FQRUSYEDT9DRWYLPLDPOBAKUSFPFFZHSKA9BADS9SJGYSY" 850
```

**Example:** "Richard" transfers all tokens back to "Karen"
```
$ cargo run  --bin transfer_value richard 150 karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/transfer_value richard 150 karen`
Bundle: "HDBZBAJUFBFZRBBSWEUGKHPGELALOH9IXJATOSNCKYFIYPH9HZJKXWARXKAKIYEEJEO9XBJSELFHVRDAD"
Transaction BKB9KXCIGEMYSEQBOKBSRSGEJSKFBRZIZKYXJ9HZWROGFBFCR9DOLHMRAUEUGLRAURGIXZADBRVBFB999 - Index 0
Transaction MTIPNGXGBYB9YCJESOTHUCUYRX9MVJCDNCZSMIJSQCRZOBACHWLRQFDJPIIWFKMVPMKLNQBTJEXPJD999 - Index 1
Transaction HREMFIFPNQHPX9G9RDSHFQKQAEIQDEGHHRWJLBGIJDJNMPPQLJKUFSMPFWQGDSEQYPFUNYDZHTTPZD999 - Index 2
```
This time only 3 transactions were generated because there is no remaining balance.
- Index 0, Tail transaction with recipient's address (value = + 150)
- Index 1, first Sender's address and its signature + (value = - 150)
- Index 2, first Sender's address and the rest of its signature (value = 0)


Richards balance is 0
```
$ cargo run  --bin get_account_info richard
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/get_account_info richard`
1 "MGDV9XKLUHPXRPOBULDSIHIYUZRCRMZUVXNFOPVDVVJEPWAIGLSJXLBZGMFLVPNZQIVDUYCGTZLGOMJCY" 0
```
**Remark**
* The first unused address ist now on Index 1


Karen's total balance is now on the first unused address on Index 2
```
$ cargo run  --bin get_account_info karen
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/get_account_info karen`
2 "XPEVRJE9JIYGKXRANSMT9WHFIJMJCW9LXU9FQRUSYEDT9DRWYLPLDPOBAKUSFPFFZHSKA9BADS9SJGYSY" 1000
```

