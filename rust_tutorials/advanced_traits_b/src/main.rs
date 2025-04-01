// A hypothetical definition of the Iterator trait using generics

pub trait Iterator<T> { 
    fn next(&mut self) -> Option<T>;
}
