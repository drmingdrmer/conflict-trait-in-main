pub trait Container {
    type Item;
}

impl Container for () {
    type Item = Foo;
}

pub struct Foo;

pub type AliasType = <() as Container>::Item;

impl Default for AliasType {
    fn default() -> Self {
        Foo
    }
}
