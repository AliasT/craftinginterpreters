use std::{borrow::BorrowMut, collections::HashMap, marker::PhantomData};

#[derive(Clone, Debug)]
struct B<'a> {
    a: i32,

    phd: PhantomData<&'a ()>,
}

#[derive(Debug)]

struct A<'a> {
    h: HashMap<i32, B<'a>>,
}

impl<'a> A<'a> {
    fn c(&self, k: &'a i32) -> &B<'a> {
        let v = self.h.get(k);
        v.unwrap()
    }
}

#[test]
fn test() {
    let mut h: HashMap<i32, B> = HashMap::new();
    h.insert(
        1,
        B {
            a: 10,
            phd: PhantomData,
        },
    );
    let mut a = A { h };

    let b = a.c(&1);

    a.h.insert(
        1,
        B {
            a: 11,
            phd: PhantomData,
        },
    );

    // b.a = 11;

    // a.b.a = 11;

    println!("{:?}", a);
    // println!("{:?}", b.a);
}
