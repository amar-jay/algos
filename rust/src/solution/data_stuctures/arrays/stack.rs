//! # Array stack
//! last-in first-out data structure
use crate::solution::data_stuctures::interface::Stack;
use std::iter;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Array Data Structure
pub struct Array<T> {
    next: Box<[Option<T>]>,
    i: usize,
    j: usize,
}

impl <T>Array<T> {

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::with_length(1)
    }

    ///Create array with a particular length
    pub fn with_length(len: usize) -> Self {
        Self {
            next: Self::allocate_in_heap(len),
            i:0,
            j:0
        }
    }

    pub fn len(&self) -> usize { self.next.len() }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        iter::repeat_with(Default::default).take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {

        let new_arr = Self::allocate_in_heap(std::cmp::max(self.i * 2, 1));
        let mut old_arr = std::mem::replace(&mut self.next, new_arr);
        for i in 0..self.i {
            self.next[i] = old_arr[(self.j + i) % old_arr.len()].take();
        }
        self.j = 0;
    }
}

impl <T: PartialEq> Array<T> {
    pub fn has(&self, j:T) -> bool {
        for i in 0..self.i {
            if self.next.get(i).unwrap().as_ref() == Some(&j) {
                return true;
            } 
        }
        false
    }
}

impl <T: Clone>Array<T> {
    pub fn own(&mut self, i: usize) -> Option<T> {
        self.next.get_mut(i)?.take()
    }
}

impl <T: Clone> Stack<T> for Array<T> {
    ///.push element to array
    fn push(&mut self, e: T) {
        if self.i + 1 >= self.len() {
            self.resize();
        }
        self.next[(self.j + self.i)% (self.len())] = Some(e);
        self.i += 1;
    }

    /// Set element in array by index
    fn pop(&mut self) -> Option<T> {
        let last = self.next[self.j].take();
        self.j = (self.j + 1)% self.len();
        self.i -= 1;

        if self.len() >= 2 * self.i {
            self.resize();
        }
        last
    }


}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::data_stuctures::interface::Stack;

    #[test]
    fn test_initialization() {
        let stack:Array<i32> = Array::new();
        assert_eq!(stack.len(), 0, "Failed array is {:?}", stack);
    }

    #[test]
    fn test_push_and_remove() {
        let mut stack:Array<char> = Array::new();
        for elem in "amar".chars() {
            stack.push(elem);
        }
        assert_eq!(stack.len(), 4);
        stack.push('j');
        stack.push('a');
        stack.push('y');
        assert_eq!(stack.len(), 7);
        for elem in "yajamar".chars() {
            assert_eq!(stack.pop(), Some(elem), "WTH is wrong :: {:?}", stack);
        }
        assert_eq!(stack.len(), 0, "WTH is wrong :: {:?}", stack);
    }

}
