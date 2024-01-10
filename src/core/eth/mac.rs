pub struct MacAddr {
    bytes: [u8; 6],
}

impl<'a> MacAddr {
    pub fn new(bytes: [u8; 6]) -> MacAddr {
        MacAddr { bytes }
    }

    // fn from_str(string: &str) -> Result<MacAddress, Box<dyn Error>> {
    //     if(Regex::new(r"^regexStr$").unwrap().is_match(string) && string.) {
    //         return Ok(MacAddress::from(string.as_bytes()))
    //     }
    // }

    pub fn bytes(self) -> [u8; 6] {
        self.bytes
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(bytes: [u8; 6]) -> MacAddr {
        MacAddr::new(bytes)
    }
}
