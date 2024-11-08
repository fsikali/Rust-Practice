/*
   Default implementation can call other methods in the same trait, 
   even if those other methods don't have a default implementation 

   - In this way a trait can provide a lot of functionality and only require
     implementors to specify a small part of it.
   
   N/B - It is not possible to call the default implementation from an overriding
         implementation of that same method.
*/

pub trait Summary { 
    fn summarize_author(&self) -> String; 

    fn summarize(&self) -> String { 
        format!("(Read more from {}...)", self.summarize_author())
    }
} 

pub struct Tweet { 
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool,
} 

impl Summary for Tweet { 
    fn summarize_author(&self) -> String { 
        format!("@{}", self.username)
    }
}   



