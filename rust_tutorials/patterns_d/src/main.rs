// Using a pattern to destructure a tuple and create three variables at once 
// Every time you've used a let statement like this you've been using patterns, although you might
// not have realized it! More formally, a let statement looks like this:
// let PATTERN = EXPRESSION;

fn main() { 
    let(x, y, z) = (1, 2, 3);
}

/*
--- Incorrectly constructing a pattern whose variables don't match the number of elements in the tuple
- Attempting to compile this code results into an error

fn main() { 
   let (x, y, z) = (1, 2, 3);
}

*/
