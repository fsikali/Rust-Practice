// Code that demonstrates the desired behavior we want our blog crate to have

use blog::Post; 

fn main() { 
    let mut post = Post::new(); 

    post.add_text("I ate a salad for lunch today"); 
    assert_eq!("", post.content()); 

    post.request_review(); 
    assert_eq!("", post.content()); 

    post.approve(); 
    assert_eq!("I ate a salad for lunch today", post.content());
}

