pub trait Add<RHS = Self>{
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}

#[derive(Debug)]
pub struct Number<T>(pub T);

impl<T: std::ops::Add<Output = T>> Add for Number<T> {
    type Output = Number<T>;
    fn add(self, other: Number<T>) -> Number<T> {
        Number(self.0 + other.0)
    }
}