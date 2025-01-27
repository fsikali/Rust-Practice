/*
- Creating instances of each of the two variants of IpAddrKind
 */
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}


