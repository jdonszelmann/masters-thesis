#[derive(Debug)]
pub struct Number {
    inner: u32,
}

impl Number {
    pub fn multiply(&self, other: u32) -> Self {
        Self {
            inner: self.inner * other
        }
    }

    pub fn square(&self) -> Self {
        self.multiply(self.inner)
    }
}












