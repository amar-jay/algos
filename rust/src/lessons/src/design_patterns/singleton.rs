#![allow(unused)]
#[derive(Clone)]
struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Singleton {
        Singleton { value: 0 }
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Mutex<Singleton> = Mutex::new(Singleton::new());
}
fn call() {
    let mut inst = INSTANCE.lock().unwrap();
    let val = inst.get_value();
    inst.set_value(val + 1);
}

fn main() {
    call();
    call();
    call();

    println!("Called {}", INSTANCE.lock().unwrap().get_value());
}
