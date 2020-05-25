
use rand::seq::SliceRandom;
use iota::{
        bundle::{Hash, 
                 Tag, 
                 Address, 
                 Transaction, 
                 TransactionField},
        ternary::{T1B1Buf,
                  TryteBuf, 
                  TritBuf},
        signing::{IotaSeed, Seed, 
                  PrivateKey, 
                  PrivateKeyGenerator, 
                  PublicKey, 
                  WotsPrivateKeyGeneratorBuilder, 
                  WotsSecurityLevel},
        crypto::{Kerl,
                 CurlP81,
                 Sponge},
};
use iota_conversion::Trinary;

//
// generate easy-to-remember seed from name
// 
pub fn generate_named_seed(mut name: String) -> String {
    name.make_ascii_uppercase();
    format!("{:9<81}", name)
}

//
// generate random seed
//
pub fn generate_random_seed() -> String {
    let input: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9".chars().collect();

    let mut list = Vec::new();
    for _ in 0..81 {
        list.push( input.choose(&mut rand::thread_rng()).unwrap() );
    }
    list.into_iter().collect::<String>()
}

//
// convert generated "seed"-String to IotaSeed
//
pub fn prepare_iota_seed(seed: &str) -> IotaSeed::<Kerl> {
    //str_to_tritbuf(&seed)
    IotaSeed::<Kerl>::from_buf(str_to_tritbuf(&seed)).unwrap()
}

//
// get address for given index
//
pub fn get_address(seed: &str, index: u64)  -> Address {
    Address::try_from_inner(
        WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
            .security_level(WotsSecurityLevel::Medium)
            .build()
            .unwrap()  // WotsPrivateKeyGenerator 
            .generate(&prepare_iota_seed(seed), index)
            .unwrap()   // PrivateKey
            .generate_public_key()
            .unwrap()  // PublicKey
            .trits()   // &Trits
            .to_owned()
    ).unwrap()
}

//
// convert Address to String (readable Address)
//
pub fn get_address_trytes(addr: &Address) -> String {
    tritbuf_to_str(addr.to_inner())   // &TritBuf<T1B1Buf>
}

//
// convert String to Hash (Trits of Transaction Hash)
// 
pub fn get_hash_trits(s: &String) -> Hash {
    Hash::try_from_inner(
        str_to_tritbuf(&s)
    ).unwrap()
}

//
// convert <Transaction> to String (Trytes of Transaction Hash)
// 
pub fn get_transaction_hash(trx: &Transaction) -> String {
    let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
    trx.into_trits_allocated(&mut trits);

    tritbuf_to_str(&CurlP81::default().digest(&trits).unwrap())   // &TritBuf<T1B1Buf>
}

//
// convert <Transaction> to String (Trytes of Transaction)
//
pub fn get_transaction_trytes(trx: &Transaction) -> String {
    let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
    trx.into_trits_allocated(&mut trits);

    tritbuf_to_str(&trits)   // &TritBuf<T1B1Buf>
}

//
// prepare tag field for struct Transfer
//     check input & 
//     convert String to Tag (Trits of Transaction Tag)
// 
pub fn prepare_tag_field(t: &str) -> Option<Tag> {

    if t.len() == 0 {
        return None
    }
    let mut tag: String = (*t).to_string();

    if t.len() < 27 {
       tag = format!("{}{}",t, "9".repeat(27 - t.len())); 
    } else if t.len() > 27 {
       tag.truncate(27);
    }

    Some(
      Tag::try_from_inner(
        str_to_tritbuf(&tag)
      ).unwrap()
    )
}

//----------------------------------------------------------------------------------------------
//
// private convinience functions
//
fn str_to_tritbuf(s: &str) -> TritBuf {
     TryteBuf::try_from_str(&s)             
         .unwrap()                     // TryteBuf Vec<Tryte> ['H', 'E', 'L', ... ]
         .as_trits()                   // &Trits<T3B1> [NegOne, PlusOne, NegOne, ... ]
         .encode::<T1B1Buf>()          // TritBuf<T1B1Buf> [NegOne, PlusOne, NegOne ... ]
}

fn tritbuf_to_str(b: &TritBuf) -> String {
        b.as_i8_slice()                // &[i8] 
         .trytes()                     // Ok(String)
         .unwrap()                     // String
}





