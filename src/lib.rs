pub trait Container {
    type Item;
}

impl Container for () {
    type Item = Foo;
}

struct Foo;

impl Default for <() as Container>::Item {
    fn default() -> Self {
        Foo
    }
}

struct Bar;

impl Default for Bar {
    fn default() -> Self {
        Bar
    }
}
