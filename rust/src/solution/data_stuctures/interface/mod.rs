//! # Trait Queue
//! ## Resources
//! -[Link to Article](https://opendatastructures.org/versions/edition-0.1c/ods-java/node5.html)

///trait for a Queue data structure
/// last-in first-out data structure
pub trait Queue<T> {
    fn add(&mut self, x:T);
    fn remove(&mut self) -> Option<T>;
}

///trait for a Stack data structure
/// first-in first-out data structure
pub trait Stack<T> {
    fn push(&mut self, x:T);
    fn pop(&mut self) -> Option<T>;
}

///trait for a List data structure
pub trait List<T> {
    fn size(&self) -> usize;
    fn add(&mut self, i:usize , x:T);
    fn remove(&mut self, i:usize) -> Option<T>;
    fn set(&mut self, i:usize, x:T) -> Option<T>;
    fn get(&self, i: usize) -> Option<T>;
}

///trait for a Unordered Set data structure
pub trait USet<T: PartialEq> {
    fn size(&self) -> usize;
    fn add(&self) -> bool;
    fn remove(&self) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
} 

///trait for a Unordered Set data structure
pub trait SSet<T: PartialEq + PartialOrd> {
    fn size(&self) -> usize;
    fn add(&self) -> bool;
    fn remove(&self) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
    fn compare(&self, x:&T, y:&T) -> bool;
} 

pub trait Graph {}
