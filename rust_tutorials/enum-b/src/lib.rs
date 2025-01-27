// Storing the data and IpAddrKind variant of an IP address using a struct

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr { 
    kind: IpAddrKind,
    address: String,
}





