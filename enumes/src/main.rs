
#![allow(unused)]
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
fn main() {
    let ip = IpAddr::V4(Ipv4Addr {});
}


//////////////////////////////////////////////////////////////////////////////////////

//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

////////////////////////////////////////////////////////////////////////////////////

// enum ip_ver {
//     V4,
//     V6,
// }

// struct ip {
//     version: ip_ver,
//     address: String,
// }

// impl ip {
//     fn new_ip(version: ip_ver, address: &str)-> Self {
//         let address = address.to_string();
//         ip { version, address }
//     }    
// }


// fn main() {
//     let ip4 = ip_ver::V4;
//     let ip6 = ip_ver::V6;
//     let home = ip::new_ip(ip_ver::V4, "127.0.0.1");
//     let loopback = ip::new_ip(ip_ver::V6, "::1");
//     what_kind(ip4);
//     what_kind(ip6);
// }

// fn what_kind(ip_: ip_ver) {}
