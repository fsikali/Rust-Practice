/*
--- Match Control Flow Construct ( match expression )
- Allows comparing of a value against a series of patterns and then execute code based on which
  pattern matches. 
- Patterns can be made up of literal values, variable names, wildcards etc. 
- 
- Example 
*/ 

enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter,
} 

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => 1, 
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Quarter => 25,
    }
}


fn value_in_cents(rect: Rectangle) -> u8 {
    match coin { 
        width => 3,
        height => 4,
    }
} 

