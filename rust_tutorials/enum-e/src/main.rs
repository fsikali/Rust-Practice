fn main() { 
    let m = Message::Write(String::from("hello"));
    m.call();
}
