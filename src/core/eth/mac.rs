use std::error::Error;
use regex::Regex;

pub struct MacAddress<'a> {
    address: &'a [u8; 6],
}

impl<'a> MacAddress<'a> {
    fn from(bytes: &[u8; 6]) -> Result<MacAddress, Box<dyn Error>> {
        Ok(MacAddress {address: bytes})
    }

    // fn from_str(string: &str) -> Result<MacAddress, Box<dyn Error>> {
    //     if(Regex::new(r"^regexStr$").unwrap().is_match(string) && string.) {
    //         return Ok(MacAddress::from(string.as_bytes()))
    //     }
    // }

    fn bytes(self: &'a Self) -> &'a [u8; 6] {
        self.address
    }
}