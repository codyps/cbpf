extern crate bit_domains;
extern crate quickcheck;
use bit_domains::Znum;
use quickcheck::{quickcheck};

/*
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
*/

#[test]
fn const_value_roundtrip() {
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
fn const_is_defined() {
    fn prop(x: u64) -> bool {
        let z = Znum::from_value(x);
        z.is_defined()
    }
    quickcheck(prop as fn(u64) -> bool);
}

#[test]
fn instance_is_defined() {
    assert_eq!(Znum::from_parts(0,0).is_defined(), false);
    assert_eq!(Znum::from_parts(0,1).is_defined(), false);
    assert_eq!(Znum::from_parts(1,1).is_defined(), false);
    assert_eq!(Znum::from_parts(1,0).is_defined(), false);
    assert_eq!(Znum::from_parts(0xfffffffffffffffe,0xfffffffffffffffe).is_defined(), false);
    assert_eq!(Znum::from_parts(0xfffffffffffffffe,0xffffffffffffffff).is_defined(), true);
    assert_eq!(Znum::from_parts(0xffffffffffffffff,0xfffffffffffffffe).is_defined(), true);
    assert_eq!(Znum::from_parts(0xffffffffffffffff,0xffffffffffffffff).is_defined(), true);
}

#[test]
fn const_bitor_is_value() {
    fn prop(x: u64, y: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);

        let c = a | b;
        c.value() == Some(x | y)
    }

    quickcheck(prop as fn(u64,u64) -> bool);
}

#[test]
fn const_bitand_is_value() {
    fn prop(x: u64, y: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);

        let c = a & b;
        c.value() == Some(x & y)
    }
    quickcheck(prop as fn(u64,u64) -> bool);
}

#[test]
fn const_xor_is_value() {
    fn prop(x: u64, y: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);

        let c = a ^ b;
        c.value() == Some(x ^ y)
    }
    quickcheck(prop as fn(u64,u64) -> bool);
}

#[test]
fn const_shl_is_value() {
    fn prop(x: u64, y: u8) -> bool {
        let a = Znum::from_value(x);
        let y = y & (64 - 1);

        let c = a << y;
        c.value() == Some(x << y)
    }
    quickcheck(prop as fn(u64,u8) -> bool);
}

#[test]
fn const_shr_is_value() {
    fn prop(x: u64, y: u8) -> bool {
        let a = Znum::from_value(x);
        let y = y & (64 - 1);

        let c = a >> y;
        c.value() == Some(x >> y)
    }
    quickcheck(prop as fn(u64,u8) -> bool);
}

#[test]
fn const_not_is_value() {
    fn prop(x: u64) -> bool {
        let a = Znum::from_value(x);
        let c = !a;
        c.value() == Some(!x)
    }
    quickcheck(prop as fn(u64) -> bool);
}

#[test]
fn union_2_const_contains_2() {
    fn prop(x: u64, y: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);

        let c = a.union(b);

        c == b.union(a) &&
            c.contains_value(x) && c.contains_value(y)
    }

    quickcheck(prop as fn(u64, u64) -> bool);
}

#[test]
fn union_3_const_contains_2() {
    fn prop(x: u64, y: u64, z: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);
        let c = Znum::from_value(z);

        let m = a.union(b);
        let n = b.union(c);
        let o = c.union(a);

        let p = m.union(c);

        p.contains(m) && p.contains(n) && p.contains(o)
    }

    quickcheck(prop as fn(u64, u64, u64) -> bool);
}

/*
/*
 * Note: this is not a valid test because Znum is permitted to contain more elements than those
 * added to it.
 *
 * It would, however, be advantagous to be able to determine reasonable exclustion style tests,
 * because otherwise it is difficult to validate that unions (and other additive operations) don't
 * over-extend the domain
 */
#[test]
fn union_3_const_contains_2_differing() {
    fn prop(x: u64, y: u64, z: u64) -> quickcheck::TestResult {
        if x == y || y == z || z == x {
            return TestResult::discard();
        }

        let a = Znum::from_value(x);
        let b = Znum::from_value(y);
        let c = Znum::from_value(z);

        let m = a.union(b);
        let n = b.union(c);
        let o = c.union(a);

        let p = m.union(c);

        let ca = p.contains(m) && p.contains(n) && p.contains(o);
        let cd = !m.contains(c) && !n.contains(a) && !o.contains(b);
        let cb = {
            !m.contains(n) && !m.contains(o) 
            && !n.contains(o) && !n.contains(m)
            && !o.contains(m) && !o.contains(n)
        };
        let cc = !m.contains(n) && !m.contains(o) 
            && !n.contains(o) && !n.contains(m)
            && !o.contains(m) && !o.contains(n);

        println!("{} {} {} {}", ca, cb, cc, cd);

        TestResult::from_bool(
            ca && cb && cc && cd
        )
    }

    quickcheck(prop as fn(u64, u64, u64) -> TestResult);
}
*/

#[test]
fn intersect_2_non_const() {
    fn prop(x: u64, y: u64) -> bool {
        let a = Znum::from_value(x);
        let b = Znum::from_value(y);

        !a.intersection(b).is_defined() || x == y
    }

    quickcheck(prop as fn(u64, u64) -> bool);
}
