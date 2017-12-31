/// Tracks which bits "may be 1s" (o) and "may be 0s" (z)
///
///
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///    https://www-apr.lip6.fr/~mine/publi/article-mine-wing12.pdf
struct Znum {
    z: u64,
    o: u64,
}

impl BitOr for Tnum {
    type Output = Tnum;
    fn bitor(self, other: Self) -> Self
    {
        Self {
            z: self.z & other.z,
            o: self.o | other.o,
        }
    }
}

impl BitAnd for Tnum {
    type Output = Tnum;
    fn bitand(self, other: Self) -> Self
    {
        Self {
            z: self.z | other.z,
            o: self.o & other.o,
        }
    }
}

impl BitXor for Tnum {
    type Output = Tnum;
    fn xor(self, other: Self) -> Self
    {
        Self {
            z: (self.z & other.z) | (self.o & other.o),
            o: (self.z & other.o) | (self.o & other.z),
        }
    }
}

/*
impl Add for Tnum {
    type Output = Tnum;
    fn add(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Sub for Tnum {
    type Output = Tnum;
    fn sub(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Neg for Tnum {
    type Output = Tnum;
    fn neg(self) -> Self {
        unimplemented!()
    }
}

impl Not for Tnum {
    type Output = Tnum;
    fn not(self) -> Self {
        unimplemented!()
    }
}

impl Mul for Tnum {
    type Output = Tnum;
    fn mul(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Div for Tnum {
    type Output = Tnum;
    fn div(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Rem for Tnum {
    type Output = Tnum;
    fn rem(self, other: Self) -> Self {
        unimplemented!()
    }
}
*/
