#[derive(Debug)]
pub struct Bar {
    a: String,
    b: usize
}

impl Bar {
    pub fn new(a: String, b: usize) -> Self{
        Self {
            a,
            b
        }
    }
}