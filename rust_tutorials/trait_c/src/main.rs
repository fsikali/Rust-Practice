use trait_c::{Summary, Tweet}; 

fn main() { 
    let tweet = Tweet { 
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"), 
        reply: false, 
        retweet: false, 
    }; 

    println!("1 new tweet: {}", tweet.summarize());
} 

