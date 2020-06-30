#[derive(Debug)]
pub struct Foo {
    x: bool,
    y: i8
}

impl Foo {
    pub fn new(x: bool, y: i8) -> Self{
        Self {
            x,
            y
        }
    }
}