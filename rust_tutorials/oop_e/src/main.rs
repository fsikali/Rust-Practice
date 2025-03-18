/*
--- Another crate using gui and implementing the Draw trait on a SelectBox struct
*/

use gui::Draw;

struct SelectBox { 
    width: u32, 
    height: u32, 
    options: Vec<String>,
}

impl Draw for SelectBox { 
    fn draw(&self) { 
        // code to actually draw a select box
    }
}

// Using trait objects to store values of different types that implement the same trait

use gui::{Butto, Screen}; 

fn main() {

    /* 
    
    // Attempting to use a type that doesn't implement the trait object's trait

    let screen = Screen { 
        components: vec![Box::new(String::from("Hi"))],
    }; 

    screen.run();

   */
  
    let screen = Screen { 
        components: vec![
            Box::new(SelectBox { 
                width: 75, 
                height: 10, 
                options: vec![ 
                    String::from("Yes"), 
                    String::from("Maybe"), 
                    String::from("No"),
                ],
            }),
            Box::new(Button { 
                width: 50, 
                height: 10, 
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
