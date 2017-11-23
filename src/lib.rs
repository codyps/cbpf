//! An eBPF implimentation
//!
//! 
//! Registers:
//! - r0 = return value
//! - r1 to r5 = arguments (caller saved, if needed)
//! - r6 to r9 = callee saved
//! - r10 = frame pointer, read only
//!
//! When a BPF program is called, `r1` contains the context.
//!
//!
//! Limits:
//!  - 4096 BPF instructions
//!  - 32 nesting calls
//!  - loops forbidden (things where termination is unprovable).

#![no_std]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Class {
    Ld =  0x00,
    Ldx = 0x01,
    St  = 0x02,
    Stx = 0x03,
    Alu = 0x04,
    Jmp = 0x05,
    Alu64 = 0x07,
}

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Src {
    K = 0x00,
    X = 0x08,
}

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum OpAlu {
    Add = 0x00,
    Sub = 0x10,
    Mul = 0x20,
    Div = 0x30,
    Or  = 0x40,
    And = 0x50,
    Lsh = 0x60,
    Rsh = 0x70,
    Neg = 0x80,
    Mod = 0x90,
    Xor = 0xa0,

    // eBPF only follow:
    Mov = 0xb0,
    Arsh= 0xc0,
    End = 0xd0,
}

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum OpJmp {
    Ja   = 0x00,
    Jeq  = 0x10,
    Jgt  = 0x20,
    Jge  = 0x30,
    Jset = 0x40,

    // eBPF only follow:
    Jne  = 0x50,
    Jsgt = 0x60,
    Jsge = 0x70,
    Call = 0x80,
    Exit = 0x90,
    Jlt  = 0xa0,
    Jle  = 0xb0,
    Jslt = 0xc0,
    Jsle = 0xd0,
}

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Size {
    W = 0x00,
    H = 0x08,
    B = 0x10,
    DW= 0x18,
}

#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Mode {
    Imm = 0x00,
    Abs = 0x20,
    Ind = 0x40,
    Mem = 0x60,

    // classic BPF only:
    Len = 0x80,
    Msh = 0xa0,

    // eBPF only:
    Xadd = 0xc0,
}

#[derive(Debug,Eq,PartialEq)]
pub enum InstDecodeError {
    InvalidOp,
    ForbiddenInst(&'static str),
    Other(&'static str)
}

#[derive(Debug,Eq,PartialEq)]
pub struct PrgmVerifyError {
    inst_idx: usize,
    inst_error: InstDecodeError,
}

#[derive(Debug,Eq,PartialEq)]
#[repr(C)]
struct Inst {
    op: u8,
    src_dst: u8,
    off: u16,
    imm: u32,
}

impl Inst {
    fn op(&self) -> u8 {
        self.op
    }

    // 4 bits msb
    fn raw_op_code(&self) -> u8 {
        self.op() & 0xf0
    }

    // 4th bit
    fn op_src(&self) -> u8 {
        self.op() & 0b0000_1000
    }

    fn raw_op_class(&self) -> u8 {
        self.op() & 0b0000_0111
    }

    fn raw_ld_mode(&self) -> u8 {
        self.op() & 0b1110_0000
    }

    fn raw_ld_size(&self) -> u8 {
        self.op() & 0b0001_1000
    }

    fn op_class(&self) -> Option<Class> {
        num_traits::FromPrimitive::from_u8(self.raw_op_class())
    }

    fn op_jmp(&self) -> Option<OpJmp> {
        num_traits::FromPrimitive::from_u8(self.raw_op_code())
    }

    fn op_alu(&self) -> Option<OpJmp> {
        num_traits::FromPrimitive::from_u8(self.raw_op_code())
    }

    fn ld_size(&self) -> Option<Size> {
        num_traits::FromPrimitive::from_u8(self.raw_ld_size())
    }

    fn ld_mode(&self) -> Option<Mode> {
        num_traits::FromPrimitive::from_u8(self.raw_ld_mode())
    }

    fn src(&self) -> u8
    {
        self.src_dst & 0xf0
    }

    fn dst(&self) -> u8
    {
        self.src_dst & 0x0f
    }

    fn off(&self) -> u16
    {
        self.off
    }

    fn imm(&self) -> u32
    {
        self.imm
    }

    fn from_parts(op: u8, src_dst: u8, off: u16, imm: u32) -> Result<Self, InstDecodeError> {
        Ok(Self {
            op: op, src_dst: src_dst, off: off, imm: imm
        })
    }

    fn from_u64(raw: u64) -> Result<Self, InstDecodeError> {
        let op      = ((raw & 0xff_00_00_00__00_00_00_00) >> (24+32)) as u8;
        let src_dst = ((raw & 0x00_ff_00_00__00_00_00_00) >> (16+32)) as u8;
        let off     = ((raw & 0x00_00_ff_ff__00_00_00_00) >> 32) as u16;
        let imm     =  (raw & 0x00_00_00_00__ff_ff_ff_ff) as u32;
        let x = Self {
            op: op, src_dst: src_dst, off: off, imm: imm
        };

        match x.op_class() {
            Some(Class::Ld) => {
                match x.ld_size() {
                    Some(Size::W) => {},
                    _ => return Err(InstDecodeError::ForbiddenInst("not W")),
                }

                match x.ld_mode() {
                    Some(Mode::Imm) => {},
                    _ => return Err(InstDecodeError::ForbiddenInst("not Imm")),
                }
            },
            Some(Class::Jmp) => {
                match x.op_jmp() {
                    Some(OpJmp::Exit) => {
                        if x.off() != 0 || x.imm() != 0 {
                            return Err(InstDecodeError::ForbiddenInst("Exit has non-zero imm or off"));
                        }
                    },
                    _ => return Err(InstDecodeError::ForbiddenInst("not Exit")),
                }
            },
            _ => return Err(InstDecodeError::ForbiddenInst("not Ld or Jmp")),
        }

        Ok(x)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Program<'a> {
    data: &'a [u64], 
}

impl<'a> Program<'a> {
    // TODO: consider construction from raw bytes so we can handle endianness internally.
    pub fn verify(data: &'a [u64]) -> Result<Self, PrgmVerifyError>
    {
        for (idx, inst) in data.iter().enumerate() {
            let _ = Inst::from_u64(*inst).map_err(|e| PrgmVerifyError {
                inst_error: e,
                inst_idx: idx
            })?;
        }

        // check that all instructions are valid encodings
        // check that we don't have any loops
        // check data flow to forbid uninitialized & out of bound reads
        // check data flow wrt context to forbid certain reads/writes


        Ok(Self { data: data })
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Context<'a> {
    prgm: Program<'a>,

    // TODO: need a way to specify the argument-registers, and the potential for them to be
    // pointers
    // TODO: also need to handle the stack/local storage which non-register arguments may be passed
    // in.
}

impl<'a> Context<'a> {
    pub fn new(prgm: Program<'a>) -> Self {
        Self {
            prgm: prgm,
        }
    }

    // TODO: note that while there is always a return value in one of the registers, the logical
    // return may not always be
    //  - the full u64 (it may be a subset).
    //  - could be a return of a larger structure via the stack, or via some context mechanism
    //  - might not have a real return-via-reg at all and instead only interact with the system via
    //  context.
    pub fn run(&self) -> u64 {
        let mut pc = 0;

        // TODO: allow restricting this to 32bit for perf?
        let mut regs = [0u64;16];
        loop {
            let i = Inst::from_u64(self.prgm.data[pc]).unwrap();
            match i.op_class() {
                Some(Class::Ld) => {
                    match i.ld_size() {
                        Some(Size::W) => {
                        },
                        _ => unreachable!(),
                    }

                    match i.ld_mode() {
                        Some(Mode::Imm) => {},
                        _ => unreachable!(),
                    }

                    regs[i.dst() as usize] = i.imm() as u64;
                },
                Some(Class::Jmp) => {
                    match i.op_jmp() {
                        Some(OpJmp::Exit) => {
                            if i.off() != 0 || i.imm() != 0 {
                                unreachable!();
                            }

                            return regs[0];
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }

            pc += 1;
        }
    }
}
