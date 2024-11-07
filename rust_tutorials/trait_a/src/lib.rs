pub trait Summary { 
    fn summarize(&self) -> String; 
} 

pub trait Shape {
    fn square_shape(&self) -> i32;
}

pub struct NewsArticle { 
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String,
} 

impl Summary for NewsArticle { 
    fn summarize(&self) -> String { 
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
} 

pub struct Tweet { 
    pub username: String, 
    pub content: String, 
    pub reply: bool,
    pub retweet: bool,
}  

pub struct Rectangle { 
    pub length: i32,
    pub width: i32,
}

impl Summary for Tweet { 
    fn summarize(&self) -> String { 
        format!("{}: {}", self.username, self.content)
    }
} 
 
impl Shape for Rectangle { 
    fn square_shape(&self) -> i32 { 
       let area: i32 = self.length * self.width;

       area
    }
}  
