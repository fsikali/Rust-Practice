/*
    Traits: Default Implementations 

    - Sometimes it's useful to have default behavior for some or all 
      of the methods in a trait instead of requiring implementations for all
      methods on every type. 
    - Then, as we implement the trait on a particular type, we can keep or 
      overide each method's default behavior. 
    
*/ 

// check on the of -- use crate::aggregator{NewsArticle, Summary}
//                 -- use aggregator::{self, NewsArticle, Summary} -- not compiling
//                 -- use aggregator::{NewsArticle, Summary}
//                 -- use crate::aggregator::{self, NewsArticle, Summary} -- not compiling

mod aggregator; 

use crate::aggregator::{NewsArticle, Summary}; 

fn main() {   

    let article = NewsArticle { 
        headline: String::from("The football game is tommorow"),
        location: String::from("City Square, Nairobi, Kenya"), 
        author: String::from("Iceburgh"),
        content: String::from( "Gor Mahia is performed well last season"),
    }; 

    println!("The magazine is available! {}", article.summarize());
}


