// Destructuring a struct's fields into separate variables

struct Point { 
    x: i32,
    y: i32,
}

fn main() { 
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; 
    assert_eq!(0, a); 
    assert_eq!(7, b);
}
