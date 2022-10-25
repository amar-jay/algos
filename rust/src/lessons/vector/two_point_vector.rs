use super::Vector;

/**
* # X - Y Vectors
* In mathematics and physics, a vector is an element of a vector space.
 *
 * The Vector2-class implements 2-dimensional vectors together with various vector-operations.
 * @see https://en.wikipedia.org/wiki/Vector_(mathematics_and_physics).
*/
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector<f32> for Vector2 {
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
    fn rotate(&self, angle: i32) -> Self {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        Vector2 {
            x: new_x,
            y: new_y,
        }
    }
}
