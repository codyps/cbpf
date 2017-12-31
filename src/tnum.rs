/// Tracking number
///
/// Tracks on a bit-by-bit level whether we know the value of a bit & what that value is (if
/// known).
///
/// References:
///  - http://bitmath.blogspot.com/2013/08/addition-in-bitfield-domain.html
///  - http://bitmath.blogspot.com/2014/02/addition-in-bitfield-domain-alternative.html
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///    https://www-apr.lip6.fr/~mine/publi/article-mine-wing12.pdf
///  - https://www.omnimaga.org/other-computer-languages-help/addition-in-the-bitfield-domain/
///
// bits in mask: 1 = unknown, 0 = known
// bits in value, if known: 1 = 1, 0 = 0
// bits in value, if unknown = 0 (iow: 1 is forbidden if bit is unknown)
#[derive(Copy,Clone,PartialEq,Eq,Debug,Default)]
struct Tnum {
    value: u64,
    mask: u64,
}

impl Tnum {
    pub fn const(value: u64) -> Self 
    {
        Self {
            value: value,
            mask: 0,
        }
    }

    // right now, we lose some information here as we are forced to expand the range to power of 2
    // might be reasonable to have direct range tracking or have a `Rnum` which extends `Tnum` with
    // range tracking.
    pub fn range(min: u64, max: u64) -> Self
    {
        unimplemented!()
    }
}

impl Default for Tnum {
    /// Default is a completely unknown value
    fn default() -> Self {
        Self {
            value: 0,
            mask: std::u64::MAX,
        }
    }
}

impl BitOr for Tnum {
    type Output = Tnum;
    fn bitor(self, other: Self) -> Self
    {
        let v1 = self.value | other.value;
        let m1 = self.mask | other.mask;
        // bit-wise saturation
        let m2 = m1 & !v1;

        Self {
            value: v1,
            mask: m2,
        }
    }
}

impl BitAnd for Tnum {
    type Output = Tnum;
    fn bitand(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

impl BitXor for Tnum {
    type Output = Tnum;
    fn bitxor(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

impl Add for Tnum {
    type Output = Tnum;
    fn add(self, other: Self) -> Self
    {
        // we need to think about how addition is handled at the bit level, but then be clever
        // about using our avaliable addition operator to reduce how much work we do.

        let mo = self.mask | other.mask;
        let ma = self.mask + other.mask;
    
        // 3 * 3 * 3 = 27 variants
        //
        // 2*2*2 = 8 are everything known
        //
        // carry.known(0) + s.known(1) + o.unknown => self unknown & carry unknown 
        // carry.known(1) + s.known(0) + o.unknown => self unknown & carry unknown
        //  really, one or the other, but unclear if we can effectively model that
        //
        // carry.known(0) + s.known(0) + o.unknown => self unknown (carry known 0)
        // carry.known(1) + s.known(1) + o.unknown => self unknown (carry known 1)
        // c.unk + s.k(0) + o.unk => s.u || c.u
        // c.unk + s.k(1) + o.unk => s.u, c.k(1)
        //
        let m1 = ma | mo; 

        let v1 = self.value + other.value;

        Self {
            mask: m1,
            value: v1
        }
    }
}

impl Sub for Tnum {
    type Output = Tnum;
    fn sub(self, other: Self) -> Self {
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
