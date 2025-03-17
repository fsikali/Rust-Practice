/*
--- Creating a channel and assigning the two halves to tx and rx
- This won't compile yet because Rust can't tell what type of values we want to send
  over the channel.
*/

use std::sync::mpsc;

fn main() { 
    let (tx, rx) = mpsc::channel();
}
