use std::collections::HashMap;

struct A<'a> {
    data: B<'a>,
}

struct B<'a> {
    s: HashMap<i32, &'a C>,
}

struct C {}

impl<'a> A<'a> {
    fn test() {
        let s = HashMap::<i32, &'a C>::new();

        let mut b = B { s };

        {
            let c = &C {};
            b.s.insert(3, c);
        }

        {
            // let f = s;
            // let c = C {};
            // b.s.insert(3, &c);
        }

        Self { data: b };
    }
}

#[test]
fn test() {}
