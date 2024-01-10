
use std::error::Error;
use std::mem;

use libc::c_char;

use libc::SOCK_RAW;
use libc::AF_PACKET;
use libc::IFNAMSIZ;
use libc::socket;

use libc::ifreq;

const DFLT_IF: &str = "eth0";

pub struct EthSocket {
    fd: usize
}

impl EthSocket {

    pub fn new(if_name: &str) -> Result<(), Box<dyn Error>> {
        let sock_fd = unsafe {
            socket(AF_PACKET, SOCK_RAW, 0)
        };

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
        }


        Ok(())
    }

//     int create_socket(char *device)
//     {	int sock_fd;
//     struct ifreq ifr;
//     struct sockaddr_ll sll;
//     memset(&ifr, 0, sizeof(ifr));
//     memset(&sll, 0, sizeof(sll));
//
//    sock_fd = socket(AF_PACKET, SOCK_RAW, htons(ETHER_TYPE));
//
//     if(sock_fd == 0) { printf("ERR: socket creation for device: %s\n", device); return FALSE; }
//
//     strncpy(ifr.ifr_name, device, sizeof(ifr.ifr_name));
//     if(ioctl(sock_fd, SIOCGIFINDEX, &ifr) == -1) { printf(" ERR: ioctl failed for device: %s\n", device); return FALSE; }
//
//     sll.sll_family      = AF_PACKET;
//     sll.sll_ifindex     = ifr.ifr_ifindex;
//     sll.sll_protocol    = htons(ETH_P_ALL);
//     if(bind(sock_fd, (struct sockaddr *) &sll, sizeof(sll)) == -1) { printf("ERR: bind failed for device: %s\n", device); return FALSE; }
//     return sock_fd;
//     }

    // pub fn send(self) -> Result {
    //     syscall!(
    //         Sysno::socket,
    //
    //     )
    // }
}