mod two_point_vector;

pub trait Vector<T> {
    fn new(x: T, y: T) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn mag(&self) -> T;
    fn normalize(&self) -> Self;
    fn rotate(&self, angle: T) -> Self;
    fn length(&self) -> T;
    fn dot(&self, other: &Self) -> T;
    fn angle_between(&self, other: &Self) -> T;
}
