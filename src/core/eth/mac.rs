use std::error::Error;
use regex::Regex;

pub struct MacAddr(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl<'a> MacAddr {

    fn new(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8) -> MacAddr {
        MacAddr(b0, b1, b2, b3,b4,b5)
    }

    fn from(bytes: [u8; 6]) -> MacAddr {
        MacAddr(bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5])
    }

    // fn from_str(string: &str) -> Result<MacAddress, Box<dyn Error>> {
    //     if(Regex::new(r"^regexStr$").unwrap().is_match(string) && string.) {
    //         return Ok(MacAddress::from(string.as_bytes()))
    //     }
    // }

    fn bytes(self) -> [u8; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }
}