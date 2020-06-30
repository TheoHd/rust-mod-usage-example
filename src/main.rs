use ::modexample::{Foo,Bar};

fn main() {
    let foo : Foo = Foo::new(true, 8);
    let bar : Bar = Bar::new(String::from("This is content for Bar."),8 as usize);
    dbg!(foo);
    dbg!(bar);
}
