//! An array List data structure
use crate::solution::data_stuctures::interface::List;
use std::iter;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Array Data Structure
pub struct Array<T> {
    next: Box<[Option<T>]>,
    n: usize,
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
            n:0,
        }
    }

    pub fn len(&self) -> usize { self.next.len() }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        iter::repeat_with(Default::default).take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_arr = Self::allocate_in_heap(std::cmp::max(self.n*2, 2));
        let old_arr = std::mem::replace(&mut self.next, new_arr);

        for (i, elem) in old_arr.into_vec().into_iter().enumerate().take(self.n) {
            self.next[i] = elem;
        }
    }
}

impl <T: PartialEq> Array<T> {
    pub fn has(&self, j:T) -> bool {
        for i in 0..self.n {
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

impl <T: Clone> List<T> for Array<T> {
    /// Get element from array by index
    fn get(&self, i: usize) -> Option<T> {
        self.next.get(i)?.as_ref().cloned()
    }

    /// Set element in array by index
    fn set(&mut self, i:usize, x:T) -> Option<T> {
        self.next.get_mut(i)?.replace(x)
    }

    /// Add element to array
    fn add(&mut self, i: usize, x:T) {
        if self.n + 1 >= self.len() {
            self.resize()
        }

        if i >= self.n {
            self.next[self.n] = Some(x);
        } else {
            self.next[i..self.n].rotate_right(1);
            let end = self.next[i].replace(x);
            self.next[self.n] = end;
        }

        self.n += 1;
    }

    /// remove element in array
    fn remove(&mut self, i:usize) -> Option<T> {
        let x = self.next.get_mut(i)?.take();

        if i < self.n {
            self.next[i..self.n].rotate_left(1);
            self.n -= 1;
            if self.len() >= 3 * self.n {
                self.resize();
            }
        }
            x

    }

    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::data_stuctures::interface::List;

    #[test]
    fn test_initialization() {
        let stack:Array<i32> = Array::new();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_add_and_set() {
        let mut stack:Array<char> = Array::new();
        for (i, elem) in "amar".chars().enumerate() {
            stack.add(i, elem);
        }
        assert_eq!((stack.size(), stack.len()), (4, 6));
        stack.add(0, 'j');
        stack.add(0, 'a');
        stack.add(0, 'y');
        assert_eq!((stack.size(), stack.len()), (7, 10));
        for (i, elem) in "yajamar".chars().enumerate() {
            assert_eq!(stack.get(i), Some(elem), "WTH is wrong :: {:?}", stack);
        }
        stack.set(0, 'x');
        assert_eq!(stack.get(0), Some('x'), "WTH is wrong :: {:?}", stack);
    }

    #[test]
    fn test_remove() {
        let mut stack:Array<char> = Array::new();
        for (i, elem) in "amar".chars().enumerate() {
            stack.add(i, elem);
        }
        assert_eq!((stack.size(), stack.len()), (4, 6));
        stack.remove(0);
        stack.remove(0);
        stack.remove(0);
        stack.remove(0);
        assert_eq!((stack.size(), stack.len()), (0, 2), "WTH is wrong :: {:?}", stack);
        stack.remove(0); // one more unneccessary remove
        assert_eq!((stack.size(), stack.len()), (0, 2), "WTH is wrong :: {:?}", stack);
    }
}
