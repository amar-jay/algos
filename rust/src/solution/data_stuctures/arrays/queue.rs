use crate::solution::data_stuctures::interface::IQueue;



#[derive(Debug, Clone)]
pub struct Queue<T> {
    vals : Vec<T>,
    len: u32,
} 

impl <T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue { vals : vec![], len: 0}
    }
    pub fn add(&mut self, val: T) -> &mut Self {
        self.vals.push(val); 
        self.len += 1;
        self
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn remove(&mut self) -> Option<T>{
        let c = self.vals.first().cloned(); 
        if self.len < 1 {
            return None;
        }
        self.vals = self.vals[1..].to_vec();
        self.len -= 1;
        c
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut a = Queue::new();
        a.add(1);
        a.add(2);
        assert_eq!(a.vals, vec![1,2]);
    }

    #[test]
    fn test_remove() {
        let mut a = Queue::new();
        a.add(1);
        a.add(2);
        a.add(9);
        assert_eq!(a.remove(),Some(1));
        assert_eq!(a.remove(),Some(2));
        assert_eq!(a.remove(),Some(9));
        assert_eq!(a.remove(),None);

    }

}
