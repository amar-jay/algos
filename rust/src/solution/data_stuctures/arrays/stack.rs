//use crate::solution::data_stuctures::interface::IStack;



#[derive(Debug, Clone)]
pub struct Stack<T> {
    vals : Vec<T>,
    len: u32,
} 

impl <T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { vals : vec![], len: 0}
    }
    pub fn push(&mut self, val: T) -> &mut Self {

        if self.len < 1 {
            self.vals.push(val);
            return self;
        }
        let mut tmp = self.vals.get(0).unwrap().clone();
        for id in 0..self.len {
            let id = id as usize;
            self.vals[id+1] = tmp;
            tmp = self.vals.get(id + 1).unwrap().clone();
        }
        self.vals[0] = val;
        self.len += 1;
        self
    }

    #[allow(dead_code)]
    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn pop(&mut self) -> Option<T>{
        /*
        let c = self.vals.last().cloned(); 
        let len = (self.len - 1).try_into();
        if let None = len.ok() {
            return None;
        };
        self.vals = self.vals[0..len.unwrap()].to_vec();
        self.len -= 1;
        c
        */
        self.vals.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut a = Stack::new();
        a.push(1);
        a.push(2);
        assert_eq!(a.vals, vec![1,2]);
    }

    #[test]
    fn test_pop() {
        let mut a = Stack::new();
        a.push(1);
        a.push(2);
        a.push(9);
        assert_eq!(a.pop(),Some(9));
        assert_eq!(a.pop(),Some(2));
        assert_eq!(a.pop(),Some(1));
        assert_eq!(a.pop(),None);

    }

}
