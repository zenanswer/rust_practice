/// Tuple-Like Structs
pub struct Bounds(pub usize, pub usize);

#[derive(Debug)]
struct PrivateBounds(usize, usize);

impl PrivateBounds {
    pub fn new(x: usize, y: usize) -> PrivateBounds {
        PrivateBounds(x, y)
    }
}

pub fn usage_private_bounds(x: usize, y: usize) -> usize {
    let pbound = PrivateBounds::new(x, y);
    println!("PrivateBounds: {:?}", pbound);
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_bounds() {
        let image_bounds = PrivateBounds(1024, 768);
        assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    }
}

#[test]
fn test_private_bounds_2() {
    let image_bounds = PrivateBounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}
