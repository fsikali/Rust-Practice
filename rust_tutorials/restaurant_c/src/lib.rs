// Making Structs and Enums Public 

// We can also use pub to designate structs and enums as public, but there are a few details 
// extra to the usage of pub with structs and enums. 

// If we use pub before a struct definition, we make the struct public, but the struct's fields will still
// be private 
// We can make each field public or not on a case-by-case basis 

// Example: Here we define a public back_of_house::Breakfast struct with a public toast field but a private
// seasonal_fruit field  

// This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, 
// but the chef decides which fruit accompanies the meal based on waht's in season and in stock 
// The available fruit changes quickly, so customers can't choose the fruit or even see which fruit they'll get

// Example: A struct with some public fields and some private fields

mod back_of_house { 
    pub struct Breakfast { 
        pub toast: String, 
        seasonal_fruit: String,
    }

    impl Breakfast { 
        pub fn summer(toast: &str) -> Breakfast { 
            Breakfast { 
                    toast: String::from(toast), 
                    seasonal_fruit: String::from("peaches"),
                }
            }
        } 
    }   

    pub fn eat_at_restaurant() { 
        // Order a breakfast in the summer with Rye toast 
        let mut meal = back_of_house::Breakfast::summer("Rye"); 

        // Change our mind about what bread we'd like 
        meal.toast = String::from("Wheat"); 
        println!("I'd like {} toast please", meal.toast); 

        // The next line won't compile if we uncomment it; we're not allowed 
        // to see or modify the seasonal fruit that comes with the meal 
        // meal.seasonal_fruit = String::from(blueberries);
}

// Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant
// we can write and read to the toast field  using dot notation. 
// N/B - That we can't use the seasonal_fruit field in eat_at_restaurant because seasonL_fruits
// is private 

// N/B - Also, note that because back_of_house::Breakfast has a private field, the struct needs to 
// provide a public associated function that constructs an instance of Breakfast (we've named it summer here) 
// If Breakfast didn't have such a function, we couldn't create an instance of Breakfasst in eat_at_restaurant 
// because we couldn't set the value of the private seasonL_fruit field in eat_at_restaurant. 


