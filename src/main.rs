// Make main.rs depends on lib.rs
use t_conflict::Container as _;

fn main() {}

struct Wow;
impl Default for Wow {
    fn default() -> Self {
        Wow
    }
}
