// The definition of the Iterator trait that has an associated type Item

pub trait Iterator { 
    type Item; 

    fn next(&mut self) -> Option<Self::Item>;
}

