// Attempting to use a refutable pattern with let

fn main() { 
    let some_option_value: Option<i32> = None; 
    let Some(x) = some_option_value;
}
