use core::ops::{Add,Sub,Mul,Rem,Div,Shl,Shr,BitXor,BitOr,BitAnd,Not};

/// Tracks which bits "may be 1s" (o) and "may be 0s" (z)
///
/// Compared to other bit domains, the Z domain requires minimal storage, which is not scaled with
/// the number of operations, but as a result the accuracy of the domain is somewhat limited.
///
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///    https://www-apr.lip6.fr/~mine/publi/article-mine-wing12.pdf
#[derive(Debug, Eq, PartialEq,Clone,Copy)]
pub struct Znum {
    z: u64,
    o: u64,
}

impl Znum {
    pub fn from_parts(ones: u64, zeros: u64) -> Self {
        Znum {
            o: ones,
            z: zeros,
        }
    }

    /// From a value, generate a Znum
    ///
    /// The resulting Znum only contains the provided value `v`, and no other values. It is
    /// considered a "constant"
    pub fn from_value(v: u64) -> Self {
        Znum {
            o: v,
            z: !v,
        }
    }

    /// Is there only a single contained value?
    pub fn is_const(&self) -> bool {
        // all const bits (differing)
        let a = self.z ^ self.o;
        // ensure all are set
        !a == 0
    }

    /// If this is a constant (only a single contained value), return that value. Otherwise, return
    /// None.
    pub fn value(&self) -> Option<u64> {
        if self.is_const() {
            Some(self.o)
        } else {
            None
        }
    }

    /// Is any value contained in this?
    ///
    /// In other words, are there _no_ undefined bits?
    pub fn is_defined(&self) -> bool {
        // all _defined_ bits
        let a = self.z | self.o;
        // ensure all are set
        !a == 0
    }

    /// Is a specific value contained in this?
    pub fn contains_value(&self, v: u64) -> bool {
        // bits provided by `ones`
        let po = self.o & v;
        // bits provided by `zeros`
        let pz = self.z & !v;
        // ensure all bits are provided
        !(po | pz) == 0
    }

    /// Return a domain containing the values is `self` and the values in `other`
    ///
    /// Addative.
    pub fn union(&self, other: Self) -> Self {
        Znum {
            o: self.o | other.o,
            z: self.z | other.z,
        }
    }

    /// `self` includes all possible elements in `other`
    pub fn contains(&self, other: Self) -> bool {
        let po = self.o & other.o;
        let pz = self.z & other.z;
        !(po | pz) == 0
    }

    /// Return a domain containing only the values that exist in both `self` and `other`.
    ///
    /// Subtractive.
    pub fn intersection(&self, other: Self) -> Self {
        Znum {
            o: self.o & other.o,
            z: self.z & other.z,
        }
    }

    /*
    /// All elements in `other` are also elements in `self`
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
        //
        // note: this special case with zero prevents us from shifting by 64bits (and getting a
        // shift overflow).
        let nz = if shift == 0 {
            0
        } else {
            ((1 << shift) - 1) << (64 - shift)
        };

        Self {
            z: self.z >> shift | nz,
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
