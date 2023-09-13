mod two_point_vector;

#[derive(Debug, PartialEq)]
pub struct Angle(f32);
impl Angle {

    pub fn cos(&self) -> f32 {
        self.0.cos()
    }
    pub fn sin(&self) -> f32 {
        self.0.sin()
    }
}
pub trait Vector<T> {
    fn new(x: T, y: T) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mag(&self) -> T;
    fn normalize(&self) -> Self;
    fn rotate(&self, angle: Angle) -> Self;
    fn length(&self) -> T;
    fn dot(&self, other: &Self) -> T;
    fn angle_between(&self, other: &Self) -> Angle;
}
