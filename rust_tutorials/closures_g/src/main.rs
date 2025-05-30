// Using sort_by_key to order rectangle by width

#[derive(Debug)] 
struct Rectangle { 
    width: u32, 
    height: u32, 
}

fn main() { 
    let mut list = [ 
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 }, 
        Rectangle { width: 7, height: 12 },
    ]; 

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
