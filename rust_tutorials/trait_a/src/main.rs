/* 
   Traits: Defining Shared Behavior
   
   - A trait defines functionality a particular type has and can share with other types.
   - We can use traits to define shared behavior in an abstract way
   - We can use trait bounds to specify that a generic type can be any type that has
     certain behavior

     N/B - Traits are similar to feature often called interfaces in other languages,
           although with some differences.
    
    Definiting a Trait - A type's behavior consists of the methods we can call on that type.
                         Different types share the same behavior if we can call the same 
                         methods on all of those types.

                         Traits definitions are a way to group method signatures together to
                         define a set of behaviors necessary to accomplish some purpose.
*/

mod aggregator; 

use aggregator::{Summary, Tweet};  

fn main() { 
    let tweet = Tweet { 
        username: String::from("horse_ebooks"), 
        content: String::from("of course, as you probaly already know people",), 
        reply: false, 
        retweet: false,
    }; 

    println!("1 new tweet: {}", tweet.summarize());
}