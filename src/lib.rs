pub trait Container {
    type Item;
}

impl Container for () {
    type Item = Foo;
}

pub struct Foo;

impl Default for <() as Container>::Item {
    fn default() -> Self {
        Foo
    }
}
