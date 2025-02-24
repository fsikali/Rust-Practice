// Hash Maps and Ownership 
// Showing that keys and values are owned by the hash map once they're inserted

use std::collections::HashMap;

fn main() {

    let field_name = String::from("Favorite color"); 
    let field_value = String::from("Blue"); 

    let mut map = HashMap::new();
    map.insert(field_name, field_value); 

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
