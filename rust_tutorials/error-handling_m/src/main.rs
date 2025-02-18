use std::net::IpAddr;

// Cases in Which You Have More Information Than the Compiler

fn main() { 
    
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address shoudl be valid");
} 
