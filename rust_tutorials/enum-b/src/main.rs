fn main() {

    let home = IpAddr { 
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }
    
    let loopback = IpAddr { 
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }
} 


