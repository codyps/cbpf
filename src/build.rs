use super::*;
use num_traits::ToPrimitive;

/*
pub struct Ld;
impl Ld {
    pub fn imm(imm: u32) -> LdImm {
        LdImm(imm)
    }
    pub fn abs(src_reg: u8, abs: u32) -> LdAbs {
        LdAbs(src_reg, abs)
    }
}
*/

fn ja(off: i16) -> u64
{
    Inst {
        op: Class::Jmp.to_u8().unwrap() | OpJmp::Ja.to_u8().unwrap(),
        src_dst: 0,
        off: off as u16,
        imm: 0,
    }.to_u64()
}

/// 
/// `*(sz *)(dst_reg + dst_off) = imm`
///
fn st_mem(sz: Size, dst_reg: u8, dst_off: i16, imm: u32) -> u64
{
    Inst {
        op: Class::St.to_u8().unwrap() | Mode::Mem.to_u8().unwrap() | sz.to_u8().unwrap(),
        src_dst: dst_reg,
        off: dst_off as u16,
        imm: imm,
    }.to_u64()
}
