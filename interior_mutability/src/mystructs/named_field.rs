use std::cell::RefCell;
use std::fmt;

/// Named-Field Structs
pub struct Point {
    pub x: i32,
    pub y: i32,
    z: i32,
    n: RefCell<i32>,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
            z: 1,
            n: RefCell::new(2),
        }
    }
    // cannot assign to `self.z` which is behind a `&` reference
    // `self` is a `&` reference, so the data it refers to cannot be written
    // pub fn pluse_z(&self) {
    pub fn pluse_z(&mut self) {
        self.z += self.dummy_private_func();
    }
    fn dummy_private_func(&self) -> i32 {
        self.z
    }
    pub fn pluse_n(&self) {
        let mut n = self.n.borrow_mut();
        *n += *n;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point(x={}, y={}, z={}, n={})",
            self.x,
            self.y,
            self.z,
            self.n.borrow()
        )
    }
}
