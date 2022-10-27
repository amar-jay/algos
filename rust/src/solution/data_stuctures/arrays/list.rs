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

    #[allow(dead_code)]
    ///Create array with a particular length
    pub fn with_length(len: usize) -> Self {
        Self {
            next: Self::allocate_in_heap(len),
            n:0,
        }
    }

    pub fn len(&self) -> usize {
        self.next.len()
    }

    #[allow(dead_code)]
    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        iter::repeat_with(Default::default).take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
         let new_a = Self::allocate_in_heap(std::cmp::max(self.n * 2, 1));
        let old_a = std::mem::replace(&mut self.next, new_a);
        for (i, elem) in old_a.into_vec().into_iter().enumerate().take(self.n) {
        self.next[i] = elem;
        }
    }
}


impl <T: PartialEq>Array<T> {}
impl <T: Clone>Array<T> {}
impl <T: Clone> List<T> for Array<T> {
    fn get(&self, i: usize) -> Option<T> {
        self.next.get(i)?.as_ref().cloned()
    }

    fn set(&mut self, i:usize, x:T) -> Option<T> {
        self.next.get_mut(i)?.replace(x)
    }

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
    fn test_add() {
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
            assert_eq!(stack.get(i), Some(elem));
        }
    }

    #[test]
    fn test_remove() {
        let mut stack:Array<char> = Array::new();
        for (i, elem) in "amar".chars().enumerate() {
            stack.add(i, elem);
        }
        assert_eq!((stack.size(), stack.len()), (4, 6));
        stack.remove(1);
        stack.remove(2);
        stack.remove(3);
        stack.remove(4);
        assert_eq!((stack.size(), stack.len()), (2, 4));
    }
}
