/*
--- Definition of a Post struct and a new function that creates a new Post instance,a State trait, and 
    a Draft struct.
*/

pub struct Post { 
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post { 
    pub fn new() -> Post { 
        Post { 
            state: Some(Box::new(Draft {})), 
            content: String::new(),
        }
    }

    // Implementing the add_text method to add text to a post's content
    pub fn add_text(&mut self, text: &str) { 
        self.content.push_str(text);
    }

    // Adding a placeholder implementation for the content method on Post that always returns an
    // empty string slice 

    pub fn content(&self) -> &str { 
        ""
    }

    // Implementing request_review methods on Post and the State trait

    pub fn request_review(&mut self) { 
        if let Some(s) = self.state.take() { 
            self.state = Some(s.request_review())
        }
    }

    // Implementing the approve method on Post and the State trait
    pub fn approve(&mut self) { 
        if let Some(s) = self.state.take() { 
            self.state = Some(s.approve())
        }
    }

    /*
    // Updating the content method on Post to delegate to a content method on State

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    */
}

trait State { 
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    
    // Adding the content method to the State trait
    fn content<'a>(&self, post: &'a Post) -> &'a str { 
        ""
    }
}

struct Draft {}

impl State for Draft { 
    fn request_review(self: Box<Self>) -> Box<dyn State> { 
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> { 
        self
    }
}

struct PendingReview {} 

impl State for PendingReview { 
    fn request_review(self: Box<Self>) -> Box<dyn State> { 
        self
    } 

    fn approve(self: Box<Self>) -> Box<dyn State> { 
        Box::new(Published {})
    }
}

struct Published {} 

impl State for Published { 
    fn request_review(self: Box<Self>) -> Box<dyn State> { 
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> { 
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str { 
        &post.content
    }
}


/*

// A post with a content method and a DraftPost without a content method

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

*/

/*

// A PendingReviewPost that gets created by calling request_review on DraftPost and an approve method that 
   turns a PendingReviewPost into a published Post

   impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

*/
