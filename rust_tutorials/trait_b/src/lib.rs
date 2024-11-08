/*
    Traits: Default Implementations 

    - Sometimes it's useful to have default behavior for some or all 
      of the methods in a trait instead of requiring implementations for all
      methods on every type. 
    - Then, as we implement the trait on a particular type, we can keep or 
      overide each method's default behavior.  
    - To use default implementation to summarize instances of NewsArticle, we 
      specify an empty impl block with impl Summary for NewsArticle {}
    - Creatign a default implementation doesn't require us to change anything about
      implementation of Summary on Tweet
    - The reason is that the syntax for overriding a default implementation is the
      same as the syntax for implementing a trait method that doesn't have a default
      implementation.
*/ 

pub trait Summary { 
    fn summarize(&self) -> String { 
        String::from("(Read more...)")
    }
} 

pub struct NewsArticle { 
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String, 
} 

impl Summary for NewsArticle {} 

pub struct Tweet { 
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool,
} 

impl Summary for Tweet { 
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
} 

