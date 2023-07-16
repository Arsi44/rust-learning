// Trait objects example

use std::fmt::Debug;

struct Values {
    a: i32,
    b: f32,
}

impl Values {
    pub fn get_debugable(&self, is_float: bool) -> &dyn Debug {
        if is_float {
            &self.b
        } else {
            &self.a
        }
    }
}

fn main() {
    let values = Values { a: 42, b: 1.4 };
    let debagable = values.get_debugable(true);
    dbg!(debagable);

    let debagables_2: [&dyn Debug; 3] = [&1 as _, &1.5 as _, &"hello" as _];
    for debagable_2 in debagables_2 {
        dbg!(debagable);
    }
}