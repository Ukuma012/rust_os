use core::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T> AddAssign for Vector2D<T>
where 
    T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}