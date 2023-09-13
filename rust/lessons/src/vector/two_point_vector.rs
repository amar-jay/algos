use super::{Vector, Angle};

/**
* # X - Y Vectors
* In mathematics and physics, a vector is an element of a vector space.
 *
 * The Vector2-class implements 2-dimensional vectors together with various vector-operations.
 * @see https://en.wikipedia.org/wiki/Vector_(mathematics_and_physics).
*/
#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector<f32> for Vector2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    fn div(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Set a vector to a magnitude of 1
    fn normalize(&self) -> Self {
        let m = self.mag();
        Vector2 {
            x: self.x / m,
            y: self.y / m,
        }
    }

    /// Find dot product b/n two vectors
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// rotate a vector to a degree. Angles measured in degrees
    fn rotate(&self, angle: Angle) -> Self {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        Vector2 {
            x: new_x,
            y: new_y,
        }
    }

    fn length(&self) -> f32 {
        self.mag()
    }

    fn angle_between(&self, other: &Self) -> Angle {
        let dot = self.dot(other);
        let mag_a = self.mag();
        let mag_b = other.mag();
        let angle = dot / (mag_a * mag_b);
        Angle(angle.acos()) 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_subtract() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        let c = Vector2::new(0.0, 0.0);
        assert_eq!(c, b.sub(&a));
    }

    #[test]
    fn test_multiply() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        let c = Vector2::new(1.0, 4.0);
        assert_eq!(c, b.mul(&a));
    }

    #[test]
    fn test_divide() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        let c = Vector2::new(1.0, 1.0);
        assert_eq!(c, b.div(&a));
    }

    #[test]
    fn test_magnitude() {
        let a = Vector2::new(3.0, 4.0);
        assert_eq!(5.0, a.mag());
    }

    #[test]
    fn test_normalize() {
        let a = Vector2::new(3.0, 4.0);
        let b = Vector2::new(0.6, 0.8);
        assert_eq!(b, a.normalize());
    }

    #[test]
    fn test_dot() {
        let a = Vector2::new(3.0, 4.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(25.0, a.dot(&b));
    }

    #[test]
    fn test_rotate() {
        let a = Vector2::new(3.0, 4.0);
        let b = Vector2::new(3.0, 4.0);
        let c = Vector2::new(-4.9202075, 0.8896954);
        assert_eq!(b, a.rotate(Angle(0.0)));
        assert_eq!(c, a.rotate(Angle(90.0)));
    }

    #[test]
    fn test_length() {
        let a = Vector2::new(3.0, 4.0);
        assert_eq!(5.0, a.length());
    }

    #[test]
    fn test_angle_between() {
        let a = Vector2::new(3.0, 4.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(Angle(0.0), a.angle_between(&b));
    }

}
