/*
   Returning Types that Implement Traits
   - We can also use the impl Trait syntax in the return position to retrun a value
     of some type that implements a trait:
   - By using impl Summary for the return type, we specify that the returns_summarizable
     function returns some type that implements the Summary trait without naming the concrete
     type.
   - In this case, returns_summarizable returns a Tweet, but the code calling this function
     doesn't need to know that.
   - The ability to specify a return type only by the trait it implements is especially useful
     in the context of closures and iterators.
   - Closures and iterators create types that only the compiler knows or the types that are very
     long to specify.
   - The impl Trait syntax lets you concisely specify that a function returns some type that
     implements the Iterator trait without needing to write out a very long type.
*/ 

pub trait Summary { 
    fn summarize(&self) -> String;
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

impl Summary for Tweet { 
    fn summarize(&self) -> String { 
        format!("{}: {}", self.username, self.content)
    }
} 

fn returns_summarizable() -> impl Summary { 
    Tweet { 
        username: String::from("horse_books"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}