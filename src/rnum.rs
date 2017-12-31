/// Range number
struct Rnum {
    max: u64,
    min: u64,

    smax: i64,
    smin: i64,
}

impl BitOr for Tnum {
    type Output = Tnum;
    fn bitor(self, other: Self) -> Self
    {
        unimplemented!()
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
    fn xor(self, other: Self) -> Self
    {
        unimplemented!()
    }
}

/*
impl Add for Tnum {
    type Output = Tnum;
    fn add(self, other: Self) -> Self {
        Self {
            max: self.max + other.max,
            min: self.min + other.min,
        }
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
