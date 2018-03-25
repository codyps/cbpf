extern crate bit_domains;
extern crate quickcheck;
use bit_domains::Znum;
use quickcheck::quickcheck;

/*
enum Op {
    Shl(u8),
    Shr(u8),
}
*/

#[test]
fn rt_const() {
    fn prop(x: u64) -> bool {
        let z = Znum::from_value(x);
        z.value() == Some(x)
    }
    quickcheck(prop as fn(u64) -> bool);
}

#[test]
fn const_contains() {
    fn prop(x: u64) -> bool {
        let z = Znum::from_value(x);
        z.contains_value(x)
    }
    quickcheck(prop as fn(u64) -> bool);
}
