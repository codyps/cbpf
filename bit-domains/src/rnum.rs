use core::ops::{Add,Sub,Mul,Rem,Div,Shl,Shr,BitXor,BitOr,BitAnd,Not};

/// Range number
pub struct Rnum {
    max: u64,
    min: u64,

    smax: i64,
    smin: i64,
}

impl BitOr for Rnum {
    type Output = Rnum;
    fn bitor(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

impl BitAnd for Rnum {
    type Output = Rnum;
    fn bitand(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

impl BitXor for Rnum {
    type Output = Rnum;
    fn bitxor(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

/*
impl Add for Rnum {
    type Output = Rnum;
    fn add(self, other: Self) -> Self {
        Self {
            max: self.max + other.max,
            min: self.min + other.min,
        }
    }
}

impl Sub for Rnum {
    type Output = Rnum;
    fn sub(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Neg for Rnum {
    type Output = Rnum;
    fn neg(self) -> Self {
        unimplemented!()
    }
}

impl Not for Rnum {
    type Output = Rnum;
    fn not(self) -> Self {
        unimplemented!()
    }
}

impl Mul for Rnum {
    type Output = Rnum;
    fn mul(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Div for Rnum {
    type Output = Rnum;
    fn div(self, other: Self) -> Self {
        unimplemented!()
    }
}

impl Rem for Rnum {
    type Output = Rnum;
    fn rem(self, other: Self) -> Self {
        unimplemented!()
    }
}
*/
