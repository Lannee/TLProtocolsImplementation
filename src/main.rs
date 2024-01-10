use tlpi::core::eth::EthSocket;

use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let sock = EthSocket::new("ens33123456789012");

    // let name: &str = "ens33";


    // let as_bytes: [libc::c_char; libc::IFNAMSIZ] = name.as_bytes().try_into()?;

    // println!(as_bytes);

    Ok(())
}