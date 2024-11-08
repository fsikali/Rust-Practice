/* 
    Returning Types that Implement Traits
    - You can only use impl Trait if you're returning a single type
    
    Example: 
    - This code that returns either a NewsArticle or a Tweet with the return type specified
      as impl Summary wouldn't work 
    - Returning either a NewsArticle or a Tweet isn't allowed due to restrictions around how
      the impl Trait syntax is implemented in the compiler.
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

fn returns_summarizable(switch: bool) -> impl Summary { 
    if switch { 
        NewsArticle { 
            headline: String::from(
                "Penguins win the Stanley Cup Championship",
            ), 
            location: String::from("Pittsburgh, PA, USA"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        } else { 
            Tweet { 
                username: String::from("horse_ebooks"),
                content: String::from( 
                    "of course, as you probably already know people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }
}