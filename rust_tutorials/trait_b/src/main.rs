// check on the of -- use crate::aggregator{NewsArticle, Summary}
//                 -- use aggregator::{self, NewsArticle, Summary} -- not compiling
//                 -- use aggregator::{NewsArticle, Summary}
//                 -- use crate::aggregator::{self, NewsArticle, Summary} -- not compiling


use trait_b::{NewsArticle, Summary}; 

fn main() {   

    let article = NewsArticle { 
        headline: String::from("The football game is tommorow"),
        location: String::from("City Square, Nairobi, Kenya"), 
        author: String::from("Iceburgh"),
        content: String::from( "Gor Mahia is performed well last season"),
    }; 

    println!("The magazine is available! {}", article.summarize());
}


