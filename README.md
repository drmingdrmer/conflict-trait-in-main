If one crate `lib.rs` implements a trait(`Default`) for an associated type(`<() as Container>::Item`, which is `Foo`),
in another crate `main.rs`, it seems the compiler treats the associated type(`<() as Container>::Item`) as a generic type `T` and assumes `T` would be any type and results in the conflict with a local type that tries to implement `Default`,
while actually `Foo` and `Wow` are actually different types.

And `impl Default for <() as Container>::Item` does not conflict with types defined in the same crate `lib.rs`


<img width="838" alt="image" src="https://github.com/user-attachments/assets/a135d57b-70fd-45c1-a1a5-a19e37310a40" />

