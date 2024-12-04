#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mul(pub u32, pub u32);

impl Mul {
    pub fn eval(&self) -> u32 {
        self.0 * self.1
    }
}
