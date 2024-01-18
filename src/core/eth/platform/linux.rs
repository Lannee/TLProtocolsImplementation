use std::mem;

use libc::c_char;

use libc::SOCK_RAW;
use libc::AF_PACKET;
use libc::IFNAMSIZ;
use libc::ETH_P_ALL;

use libc::socket;
use libc::ioctl;
use libc::bind;
use libc::write;

use libc::SIOCGIFINDEX;
use libc::SIOCGIFHWADDR;

use libc::ifreq;
use libc::sockaddr_ll;
use libc::sockaddr;

use crate::core::eth::eth_frame::EthHeader;
use crate::core::eth::mac::MacAddr;


const DFLT_IF: &str = "eth0";

pub struct EthSocket {
    fd: i32,
    eth_hdr: EthHeader 
}

impl EthSocket {

    pub fn new(if_name: &str) -> Result<EthSocket, &str> {
        let sock_fd = unsafe {
            socket(AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be()) as i32
        };

        if sock_fd < 0 {return Err("socket creation for device");}

        println!("socket fd: {sock_fd}");

        unsafe {
            let mut ifr: ifreq = mem::zeroed();

            let mut arr : [c_char; IFNAMSIZ] = [0;IFNAMSIZ];
            let mut i = 0;
            let if_name_bytes = if_name.as_bytes();

            while i < IFNAMSIZ - 1 {
                arr[i] = match if_name_bytes.get(i) {
                    Some(val) => *val as c_char,
                    None => '\0' as c_char
                };

                i += 1;
            }

            arr[i] = '\0' as c_char;
            ifr.ifr_name = arr;

            if ioctl(sock_fd, SIOCGIFINDEX, &ifr) == -1 {return Err("ioctl failed for device");}


            let mut sll: sockaddr_ll = mem::zeroed();

            sll.sll_family      = AF_PACKET as libc::c_ushort;
            sll.sll_ifindex     = ifr.ifr_ifru.ifru_ifindex;
            sll.sll_protocol    = ETH_P_ALL.to_be() as libc::c_ushort;

            if bind(sock_fd, 
                &sll as *const sockaddr_ll as *const sockaddr, 
                mem::size_of::<sockaddr_ll>() as libc::socklen_t) < 0 { 
                    return Err("bind failed for device");
                }

            if ioctl(sock_fd, SIOCGIFHWADDR, &ifr) == -1 {return Err("ioctl failed for device");}

            Ok(EthSocket {
                fd: sock_fd,
                eth_hdr: EthHeader::new(
                    MacAddr::zeroed(),
                    // MacAddr::zeroed(),
                    MacAddr::from( {
                        let mut u8_arr: [u8; 6] = [0; 6];

                        for i in 0..6 {
                            u8_arr[i] = ifr.ifr_ifru.ifru_hwaddr.sa_data[i] as u8
                        }
                        u8_arr
                    }
                            // <[i8; 6]>::try_from(
                            //     &ifr.ifr_ifru.ifru_hwaddr.sa_data[..6]
                            // ).unwrap().iter().map(|&x| x as u8).collect::<[u8; 6]>()
                    ),
                    0 as u16
                )
            })
        }


    }

    pub fn send(&self, data: &[u8]) -> Result<(), &str>  {
        let data = self.

        let write_res = unsafe {
            write(
                self.fd as libc::c_int,
                data.as_ptr() as *const libc::c_void,
                data.len() as libc::size_t
            )
        };

        if write_res < 0 {
            return Err("write error")
        }

        Ok(())
    }
}