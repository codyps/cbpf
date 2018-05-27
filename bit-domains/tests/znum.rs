extern crate bit_domains;
extern crate quickcheck;
use bit_domains::Znum;
use quickcheck::quickcheck;
use std::ops;
use ops::{Add,Sub,Mul,Rem,Div,Shl,Shr,BitXor,BitOr,BitAnd,Not};

#[derive(Debug,Eq,PartialEq,Clone)]
enum ConstOp {
    Shl(u8),
    Shr(u8),
    BitXor(u64),
    BitOr(u64),
    BitAnd(u64),
    Not,
    /*
    Add(u64),
    Sub(u64),
    Neg,
    */
}

impl ConstOp {
    fn run<T: (
}

impl quickcheck::Arbitrary for ConstOp {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let c = g.next_u64();
        let o = [
            ConstOp::Shl(c as u8),
            ConstOp::Shr(c as u8),
            ConstOp::BitXor(c),
            ConstOp::BitOr(c),
            ConstOp::BitAnd(c),
            ConstOp::Not
        ];

        g.choose(&o[..]).unwrap().clone()
    }
}

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

#[test]
