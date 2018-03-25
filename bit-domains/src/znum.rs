use core::ops::{Add,Sub,Mul,Rem,Div,Shl,Shr,BitXor,BitOr,BitAnd,Not};

/// Tracks which bits "may be 1s" (o) and "may be 0s" (z)
///
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///    https://www-apr.lip6.fr/~mine/publi/article-mine-wing12.pdf
pub struct Znum {
    z: u64,
    o: u64,
}

impl Znum {
    pub fn from_value(v: u64) -> Self {
        Znum {
            o: v,
            z: !v,
        }
    }

    /// Is the value a constant?
    pub fn is_const(&self) -> bool {
        // all const bits (differing)
        let a = self.z ^ self.o;
        // ensure all are set
        !a == 0
    }

    pub fn value(&self) -> Option<u64> {
        if self.is_const() {
            Some(self.o)
        } else {
            None
        }
    }

    /// Does a value exist in this domain?
    ///
    /// In other words, are there _no_ undefined bits?
    pub fn is_defined(&self) -> bool {
        // all _defined_ bits
        let a = self.z | self.o;
        // ensure all are set
        !a == 0
    }

    pub fn contains_value(&self, v: u64) -> bool {
        // bits provided by `ones`
        let po = self.o & v;
        // bits provided by `zeros`
        let pz = self.z & !v;
        // ensure all bits are provided
        !(po | pz) == 0
    }

    /*
    pub fn intersect(&self, other: Self) -> Self {

    }

    pub fn union(&self, other: Self) -> Self {

    }

    pub fn is_subset(&self, other: Self) -> bool {

    }
    */

    /*
    fn from_range(low: u64, high: u64) -> Self {

    }
    */
}

impl BitOr for Znum {
    type Output = Znum;
    fn bitor(self, other: Self) -> Self
    {
        Self {
            z: self.z & other.z,
            o: self.o | other.o,
        }
    }
}

impl BitAnd for Znum {
    type Output = Znum;
    fn bitand(self, other: Self) -> Self
    {
        Self {
            z: self.z | other.z,
            o: self.o & other.o,
        }
    }
}

impl BitXor for Znum {
    type Output = Znum;
    fn bitxor(self, other: Self) -> Self
    {
        Self {
            z: (self.z & other.z) | (self.o & other.o),
            o: (self.z & other.o) | (self.o & other.z),
        }
    }
}

impl Not for Znum {
    type Output = Znum;
    fn not(self) -> Self {
        Self {
            z: self.o,
            o: self.z,
        }
    }
}

impl Shl<u8> for Znum {
    type Output = Znum;
    fn shl(self, shift: u8) -> Self {
        // ones move up, zeros move up, empty space filled by zeros
        Self {
            z: self.z << shift | ((1 << shift) - 1),
            o: self.o << shift,
        }
    }
}

impl Shr<u8> for Znum {
    type Output = Znum;
    fn shr(self, shift: u8) -> Self {
        // ones move down, zeros move down, empty space filled by zeros
        Self {
            z: self.z >> shift | (((1 << shift) - 1) << (64 - shift)),
            o: self.o >> shift,
        }
    }
}

/*
impl Add for Znum {
    type Output = Znum;
    fn add(self, other: Self) -> Self {
        /*
         * 1 bit addition truth table:
         *
         * o1z1o2z2O Z
         * 0 0 0 0 0 0
         * 0 0 0 1 0 0
         * 0 0 1 0 0 0
         * 0 0 1 1 0 0
         * 0 1 0 0 0 0
         * 0 1 0 1 0 1
         * 0 1 1 0 1 0
         * 0 1 1 1 1 1
         * 1 0 0 0 0 0
         * 1 0 0 1 1 0
         * 1 0 1 0 0 1
         * 1 0 1 1 1 1
         * 1 1 0 0 0 0
         * 1 1 0 1 1 1
         * 1 1 1 0 1 1
         * 1 1 1 1 1 1
         */


        /*
         * +1:
         *   o: self.o + 1 | ((self.o ^ self.z) & 1)
         *   z: self.z
         *
         *  if self.o & 1 == 1
         *
         * +2:
         *   o
         */


        Self {
            o: self.o + other.o,
        }
    }
}
*/


/*
impl Sub for Znum {
    type Output = Znum;
    fn sub(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Neg for Znum {
    type Output = Znum;
    fn neg(self) -> Self {
        unimplemented!()
    }
}

impl Mul for Znum {
    type Output = Znum;
    fn mul(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Div for Znum {
    type Output = Znum;
    fn div(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Rem for Znum {
    type Output = Znum;
    fn rem(self, other: Self) -> Self {
        unimplemented!()
    }
}
*/
