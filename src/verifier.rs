use super::*;

use core::{cmp, usize, convert::From};

#[derive(Debug,Eq,PartialEq)]
pub enum PrgmVerifyErrorKind {
    /// Instruction was not decodable
    InstDecode(InstDecodeError),
    /// Attempted to verify non-existant location
    InvalidInstIdx,
    /// Tried to load a program that exceeds the instruction limit
    InstLimitExceeded,
    /// 
    Other(&'static str),
}

#[derive(Debug,Eq,PartialEq)]
pub struct PrgmVerifyError {
    inst_idx: usize,
    kind: PrgmVerifyErrorKind,
}

impl From<InstDecodeError> for PrgmVerifyErrorKind
{
    fn from(v: InstDecodeError) -> Self {
        PrgmVerifyErrorKind::InstDecode(v)
    }
}

impl From<(usize, InstDecodeError)> for PrgmVerifyError
{
    fn from(v: (usize, InstDecodeError)) -> Self {
        PrgmVerifyError {
            inst_idx: v.0,
            kind: From::from(v.1),
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
enum RegType {
    NotInit,
    Value,
    Ptr,
}

#[derive(Clone,PartialEq,Eq,Debug)]
enum RegLiveness {
    No,
    Read,
    Write,
}

#[derive(Clone,PartialEq,Eq,Debug)]
struct RegState {
    ty: RegType,

    /*
    smin: i64,
    smax: i64,
    umin: u64,
    umax: u64,
    val: Tnum,
    */

    live: RegLiveness,
}

impl Default for RegState {
    fn default() -> Self {
        Self {
            ty: RegType::NotInit,
            live: RegLiveness::No,
        }
    }
}

impl RegState {
    fn load_imm(&mut self, imm: u64)
    {
        //self.umax = cmp::max(imm, self.umax);
        self.ty = RegType::Value;
        self.live = RegLiveness::Write;
    }
}

#[derive(Debug,PartialEq,Eq,Default)]
struct State {
   regs: [RegState;10],
}

impl State {
//    fn call(&mut self)
}

/// The environemnt a BPF program is invoked in, describes the limitations/requirements on that BPF
/// program.
///
/// Currently only provides a instruction limit.
#[derive(Debug,PartialEq,Eq,Default)]
struct Env {
    //states: Vec<State>,
    inst_limit: Option<usize>,
}

// A basic block of the 
pub struct Block {
    start: usize,
    end: usize,
}

impl Env {
    pub fn with_inst_limit(inst_limit: usize) -> Self
    {
        Self {
            inst_limit: Some(inst_limit),
            ..Default::default()
        }
    }

    // TODO: consider construction from raw bytes so we can handle endianness internally.
    pub fn verify<'a>(&mut self, data: &'a [u64]) -> Result<Program<'a>, PrgmVerifyError>
    {
        let inst_ct = data.len();

        // check that all instructions are valid encodings
        for (idx, inst) in data.iter().enumerate() {
            let _ = Inst::from_u64(*inst).map_err(|e| PrgmVerifyError {
                kind: From::from(e),
                inst_idx: idx
            })?;
        }

        // check that we don't have any loops
        // check data flow to forbid uninitialized & out of bound reads
        // check data flow wrt context to forbid certain reads/writes
        // check that the return value (if any) is initialized
        // forbid dead stores (?)

        // simulation can tell us what registers require an initial value
        // alternately, us saying "these will be the initial values" could simplify validation in
        // simulation.

        let mut pc = 0;
        loop {
            if pc > inst_ct {
                return Err(PrgmVerifyError {
                    kind: PrgmVerifyErrorKind::InvalidInstIdx,
                    inst_idx: pc
                });
            }

            if pc > self.inst_limit.unwrap_or(usize::MAX) {
                return Err(PrgmVerifyError {
                    kind: PrgmVerifyErrorKind::InstLimitExceeded,
                    inst_idx: pc
                });
            }

            let i = Inst::from_u64(data[pc]).unwrap();

            match i.op_class() {
                Some(Class::Ld) => {
                    match i.ld_mode() {
                        Some(Mode::Imm) => {
                            if i.off16() != 0 {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::InvalidEncoding("ld.imm has offs != 0")
                                )));
                            }

                            if i.ld_size() == Some(Size::DW) {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::ForbiddenInst("ld.imm.dw(64) not implimented")
                                )));
                            }
                        },
                        Some(Mode::Abs) => {
                            if i.off16() != 0 {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::InvalidEncoding("ld.abs has offs != 0")
                                )));

                            }
                            if i.src() != 0 {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::InvalidEncoding("ld.abs has src_reg != 0")
                                )));
                            }
                        },
                        Some(Mode::Ind) => {
                            if i.off16() != 0 {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::InvalidEncoding("ld.ind has offs != 0")
                                )));
                            }
                        }
                        _ => return Err(From::from((
                                    pc,
                                    InstDecodeError::ForbiddenInst("invalid Ld mode")
                        ))),
                    }
                },
                Some(Class::Ldx) => {
                    match i.ld_mode() {
                        Some(Mode::Mem) => {
                        },
                        _ => return Err(From::from((
                                    pc,
                                    InstDecodeError::ForbiddenInst("invalid Ld mode")
                        ))),
                    }
                },
                Some(Class::Jmp) => {
                    match i.op_jmp() {
                        Some(OpJmp::Exit) => {
                            if i.off16() != 0 || i.imm32() != 0 {
                                return Err(From::from((
                                            pc,
                                            InstDecodeError::ForbiddenInst("Exit has non-zero imm or off")
                                )));
                            }
                        },
                        Some(OpJmp::Ja) => {
                            // jump always
                            unimplemented!()
                        },
                        _ => return Err(From::from((
                                    pc,
                                    InstDecodeError::ForbiddenInst("not Exit")
                        ))),
                    }
                },
                _ => return Err(From::from((
                            pc,
                            InstDecodeError::ForbiddenInst("not Ld or Jmp")
                ))),
            }

            pc += 1;
        }

        Ok(unsafe { Program::from_raw(data) })
    }
}
