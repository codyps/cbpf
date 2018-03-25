extern crate bit_domains;
extern crate quickcheck;
use bit_domains::Znum;
use quickcheck::quickcheck;

fn prop(x: u64) -> bool {
    let z = Znum::from_value(x);
    z.value() == Some(x)
}

#[test]
fn rt_const() {
    quickcheck(prop as fn(u64) -> bool);
}
