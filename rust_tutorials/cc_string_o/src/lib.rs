// The bytes method returns each raw byte, which might be appropriate for domain:

#![allow(unused)]
fn main() { 
    for b in "Зд".bytes() { 
        println!("{b}");
    }
}
