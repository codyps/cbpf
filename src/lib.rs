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

// TODO: can this be made performant for 32-bit systems?
// TODO: can we make sanitize mode? (iow: verify on run)

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod build;
mod verifier;
//mod buffer;

//mod tnum;
//pub use tnum::Tnum;

/// Broad class that an instruction fits into
///
/// The upper 4-bits of the `opcode`.
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Class {
    Ld =  0x00,
    /// Load, using a register as the destination
    Ldx = 0x01,
    /// Store, using the immediate as the source
    St  = 0x02,
    /// Store using a register as the source
    Stx = 0x03,

    /// Arithmetic in 32 bits
    Alu = 0x04,

    /// Conditional & unconditional jumps
    Jmp = 0x05,

    /// Arithmetic in 64 bits
    Alu64 = 0x07,
}

/// Use either immediate or registers as the source
///
/// Part of the `opcode` for `Class:Alu`, `Class::Alu64`, and `Class:Jmp`
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Src {
    /// Immediate
    K = 0x00,
    /// Registers
    X = 0x08,
}

/// Ops for `Class::Alu` and `Class::Alu64`
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum OpAlu {
    /// `D += S`
    Add = 0x00,
    /// `D -= S`
    Sub = 0x10,
    /// `D *= S`
    Mul = 0x20,
    /// `D /= S`
    Div = 0x30,
    /// `D |= S`
    Or  = 0x40,
    /// `D &= S`
    And = 0x50,
    /// Left shift
    Lsh = 0x60,
    /// Logical right shift
    Rsh = 0x70,
    /// ?
    Neg = 0x80,
    /// `D %= S`
    Mod = 0x90,
    /// `D ^= S`
    Xor = 0xa0,

    // eBPF only follow:
    /// Move
    Mov = 0xb0,
    /// Arithmetic right shift
    Arsh= 0xc0,
    /// Endianness conversion
    End = 0xd0,
}

/// Ops for `Class::Jmp`
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum OpJmp {
    /// jump always
    Ja   = 0x00,
    Jeq  = 0x10,
    Jgt  = 0x20,
    Jge  = 0x30,
    Jset = 0x40,

    // eBPF only follow:
    /// Jump if not equal
    Jne  = 0x50,
    /// Jump if signed greater than
    Jsgt = 0x60,
    /// Jump if signed greater than or equal to
    Jsge = 0x70,
    Call = 0x80,
    Exit = 0x90,
    Jlt  = 0xa0,
    Jle  = 0xb0,
    Jslt = 0xc0,
    Jsle = 0xd0,
}

/// Size for `Class::St`, `Class::Stx`, `Class:Ld`, and `Class::Ldx`
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Size {
    /// u32, "word"
    W = 0x00,
    /// u16, "half word"
    H = 0x08,
    /// u8, "byte"
    B = 0x10,
    /// u64, "double word"
    DW= 0x18,
}

/// Mode for `Class::St`, `Class::Stx`, `Class:Ld`, and `Class::Ldx`
///
/// Indicates where the meaning of the destination
#[derive(Debug,Eq,PartialEq,Primitive)]
#[repr(u8)]
enum Mode {
    /// Load the immidate into register indicated by `dst`.
    ///  `off` & `src` must be zeroed
    Imm = 0x00,

    /// Special: absolute load from the data area
    ///
    /// `Class::Ld` only
    ///
    /// `r0 = *(T *)(DATA_AREA + imm32)`
    Abs = 0x20,

    /// Special: indirect load from the data area
    ///
    /// `Class::Ld` only.
    ///
    /// `r0 = *(T *)(DATA_AREA + src_reg + imm32)`
    Ind = 0x40,
    
    /// Normal memory access
    ///
    /// `Class::Ld`/`Class::Ldx`:
    ///   Read the memory pointed to by the src register/immediate + offset, and store to dest register
    ///
    /// `Class:St`/`Class::Stx`:
    ///   Write the memory pointed to by the dst register/immediate + offset with the value from the
    ///   src register
    Mem = 0x60,

    /// Classic BPF only
    Len = 0x80,
    /// Classic BPF only
    Msh = 0xa0,

    /// Exclusive add, eBPF only
    Xadd = 0xc0,
}

#[derive(Debug,Eq,PartialEq)]
pub enum InstDecodeError {
    InvalidEncoding(&'static str),
    ForbiddenInst(&'static str),
    Other(&'static str)
}

/// An instruction split into rough fields.
#[derive(Debug,Eq,PartialEq)]
#[repr(C)]
struct Inst {
    /// layout for ld/st:
    /// 
    ///   +- 3b -+- 2b -+-- 3b -+
    ///   | mode | size | class |
    ///
    /// layout for jmp/alu:
    ///
    ///   +- 4b -+- 1b -+-- 3b -+
    ///   | code | src  | class |
    op: u8,
    src_dst: u8,
    off: u16,
    imm: u32,
}

impl Inst {
    fn op(&self) -> u8 {
        self.op
    }

    /// Examined for all instructions
    fn raw_op_class(&self) -> u8 {
        self.op() & 0b0000_0111
    }

    /// Alu & Jmp only
    // 4 bits msb
    fn raw_op_code(&self) -> u8 {
        self.op() & 0xf0
    }

    /// Alu & Jmp Only
    // 4th bit
    fn raw_op_src(&self) -> u8 {
        self.op() & 0b0000_1000
    }

    /// Ld & St only
    fn raw_ld_mode(&self) -> u8 {
        self.op() & 0b1110_0000
    }

    /// Ld & St only
    fn raw_ld_size(&self) -> u8 {
        self.op() & 0b0001_1000
    }

    fn op_src(&self) -> Option<Src> {
        num_traits::FromPrimitive::from_u8(self.raw_op_src())
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
        (self.src_dst & 0xf0) >> 4
    }

    fn dst(&self) -> u8
    {
        self.src_dst & 0x0f
    }

    fn off16(&self) -> i16
    {
        self.off as i16
    }

    fn imm32(&self) -> u32
    {
        self.imm
    }

    fn from_raw_parts(op: u8, src_dst: u8, off: u16, imm: u32) -> Result<Self, InstDecodeError> {
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

        Ok(x)
    }

    fn to_u64(&self) -> u64
    {
        ((self.op as u64) << (24+32))
            | ((self.src_dst as u64) << (16+32))
            | ((self.off as u64) << 32)
            | (self.imm as u64)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Program<'a> {
    data: &'a [u64], 
}

impl<'a> Program<'a> {
    pub unsafe fn from_raw(data: &'a [u64]) -> Self {
        Self {
            data: data
        }
    }
}

/// A DataArea provides a region of memory from which sized data may be loaded
///
/// All accesses to the `DataArea` should be checked. Accesses that fail (return `None`) cause the
/// accessing `Invoke` to terminate (return an error).
pub trait DataArea {
    fn load_u64(&self, offs: usize) -> Option<u64>;
    fn load_u32(&self, offs: usize) -> Option<u32>;
    fn load_u16(&self, offs: usize) -> Option<u16>;
    fn load_u8 (&self, offs: usize) -> Option<u8>;
}

/// A `DataArea` for which accesses always fail
pub struct EmptyDataArea;

impl DataArea for EmptyDataArea {
    fn load_u64(&self, _:usize) -> Option<u64> { None }
    fn load_u32(&self, _:usize) -> Option<u32> { None }
    fn load_u16(&self, _:usize) -> Option<u16> { None }
    fn load_u8(&self, _:usize) -> Option<u8> { None }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Invoke<'a, D: DataArea> {
    prgm: Program<'a>,

    // TODO: need a way to specify the argument-registers, and the potential for them to be
    // pointers
    // TODO: also need to handle the stack/local storage which non-register arguments may be passed
    // in.
 
    regs: [u64;16],
    data_area: D,
}

impl<'a> Invoke<'a, EmptyDataArea> {
    pub fn new(prgm: Program<'a>) -> Invoke<'a, EmptyDataArea> {
        Self::with_data_area(prgm, EmptyDataArea)
    }
}

impl<'a, D: DataArea> Invoke<'a, D> {
    pub fn with_data_area(prgm: Program<'a>, data_area: D) -> Self {
        Self {
            prgm: prgm,
            regs: Default::default(),
            data_area: data_area,
        }
    }

    // this API is _bad_
    pub fn arg_raw(&mut self, reg: usize, val: u64) {
        self.regs[reg] = val; 
    }

    fn data_area_load(&mut self, offs: usize, sz: Option<Size>) -> Option<u64> {
        match sz {
            Some(Size::W) => {
                self.data_area.load_u32(offs).map(|x| x as u64)
            },
            Some(Size::H) => {
                self.data_area.load_u16(offs).map(|x| x as u64)
            },
            Some(Size::B) => {
                self.data_area.load_u8(offs).map(|x| x as u64)
            },
            Some(Size::DW) => {
                self.data_area.load_u64(offs).map(|x| x as u64)
            },
            None => panic!(),
        }
    }

    // TODO: note that while there is always a return value in one of the registers, the logical
    // return may not always be
    //  - the full u64 (it may be a subset).
    //  - could be a return of a larger structure via the stack, or via some context mechanism
    //  - might not have a real return-via-reg at all and instead only interact with the system via
    //  context.
    pub fn run(mut self) -> Result<u64, ()> {
        let mut pc = 0;

        // TODO: allow restricting this to 32bit for perf?
        // TODO: should this be allocated per-run?
        loop {
            let i = Inst::from_u64(self.prgm.data[pc]).unwrap();
            match i.op_class() {
                Some(Class::Ld) => {
                    match i.ld_mode() {
                        Some(Mode::Imm) => {
                            // check: i.src() == 0
                            // check: i.off16() == 0

                            match i.ld_size() {
                                Some(Size::W) => {
                                    self.regs[i.dst() as usize] = i.imm32() as u64;
                                },
                                Some(Size::H) => {
                                    self.regs[i.dst() as usize] = i.imm32() as u64;
                                },
                                Some(Size::B) => {
                                    self.regs[i.dst() as usize] = i.imm32() as u64;
                                },
                                Some(Size::DW) => {
                                    panic!("ld.imm.64 is not implimented");
                                    self.regs[i.dst() as usize] = i.imm32() as u64;
                                    // ???
                                },
                                _ => panic!(),
                            }
                        },
                        Some(Mode::Abs) => {
                            let offs = i.imm32() as usize;
                            self.regs[i.dst() as usize] = self.data_area_load(offs, i.ld_size()).ok_or(())?;
                        },
                        Some(Mode::Ind) => {
                            let offs = i.imm32() as usize + self.regs[i.src() as usize] as usize;
                            self.regs[i.dst() as usize] = self.data_area_load(offs, i.ld_size()).ok_or(())?;
                        },
                        _ => panic!(),
                    }
                },
                /*
                Some(Class::Stx) => {
                    match i.ld_mode() {
                        Some(Mode::Mem) => {}
                    }
                },
                Some(Class::St) => {
                    match i.ld_mode() {
                        Some(Mode::Mem) => {}
                    }
                },
                */
                Some(Class::Jmp) => {
                    // FIXME: we don't always need to `a` & `b`. `Call` and `Exit` don't use them. 
                    let a = self.regs[i.dst() as usize];
                    let b = match i.op_src() {
                        // immediate
                        Some(Src::K) => {
                            // check: i.src() == 0
                            i.imm32() as u64
                        },
                        // register
                        Some(Src::X) => {
                            // check: i.imm32() == 0
                            self.regs[i.src() as usize]
                        },
                        None => panic!(),
                    };

                    let jmp = match i.op_jmp() {
                        Some(OpJmp::Ja) => {
                            // check: pc + i.off16() < instruction_ct
                            // check: src_dst == 0
                            true
                        },
                        Some(OpJmp::Jeq) => {
                            // check: pc + i.off16() < instruction_ct
                            // check: i.src() != i.dst()
                            a == b
                        },
                        Some(OpJmp::Jgt) => {
                            // check: pc + i.off16() < instruction_ct
                            // check: i.src() != i.dst()
                            a > b
                        },
                        Some(OpJmp::Jge) => {
                            // check: pc + i.off16() < instruction_ct
                            // check: i.src() != i.dst()
                            a >= b
                        },
                        Some(OpJmp::Jset) => {
                            (a & b) != 0
                        },

                        // eBPF only follow:
                        Some(OpJmp::Jne) => {
                            a != b
                        },
                        Some(OpJmp::Jsgt) => {
                            (a as i64) > (b as i64)
                        },
                        Some(OpJmp::Jsge) => {
                            (a as i64) >= (b as i64)
                        },
                        Some(OpJmp::Call) => {
                            // push `pc+1` on the return stack, and jump to i.imm32()
                            // TODO: we don't currently support calling
                            panic!()
                        },
                        Some(OpJmp::Exit) => {
                            // check: i.off16() == 0
                            // check: i.imm32() == 0
                            return Ok(self.regs[0]);
                        },

                        Some(OpJmp::Jlt)  => {
                            a < b
                        },
                        Some(OpJmp::Jle)  => {
                            a <= b 
                        },
                        Some(OpJmp::Jslt) => {
                            (a as i64) < (b as i64)
                        },
                        Some(OpJmp::Jsle) => {
                            (a as i64) <= (b as i64)
                        },
                        None => panic!(),
                    };

                    if jmp {
                        pc += i.off16() as usize;
                    }
                },
                /*
                Some(Class::Alu) => {
                    match i.op_alu() {
                        _ => panic!(),
                    }
                },
                Some(Class::Alu64) => {
                    match i.op_alu() {
                        _ => panic!(),
                    }
                },
                */
                _ => panic!(),
            }

            pc += 1;
        }
    }
}
