extern crate cbpf;

/*
fn run_raw(prgm: &[u64]) -> usize
{
    let p = unsafe { cbpf::Program::from_raw(&prgm[..]) };
    let c = cbpf::Invoke::new(p);
    c.run()
}
*/

#[test]
fn ret() {
    let r = [
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_01,
        // exit
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x1);
}

#[test]
fn ret2() {
    let r = [
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        // exit
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x2);
}

#[test]
fn ja() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_01,
        // ja +1
        0x05_00_00_01__00_00_00_00,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x1);
}

#[test]
fn je_imm_true() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__DEADBEEF,
        // je #0xDEADBEEF, r0, 1
        0x15_00_00_01__DEADBEEF,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0xDEADBEEF);
}

#[test]
fn je_imm_false() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__DEADBEEF,
        // je #0xDEADBEEE, r0, 1
        0x15_00_00_01__DEADBEEE,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x2);
}

#[test]
fn jg_imm_false() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_10,
        // jg #0x10, r0, 1
        0x25_00_00_01__00_00_00_10,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x2);
}

#[test]
fn jg_imm_true() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_10,
        // jg #0x10, r0, 1
        0x25_00_00_01__00_00_00_09,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x10);
}

#[test]
fn jge_imm_true() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_10,
        // jset #0x10, r0, 1
        0x35_00_00_01__00_00_00_10,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x10);
}

#[test]
fn jge_imm_false() {
    let r = [
        // OP_SRCDST_OFF__IMM
        // ld r0, 0x1u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_10,
        // jg #0x10, r0, 1
        0x35_00_00_01__00_00_00_11,
        // ld r0, 0x2u32
        //  LD|MEM|W
        0x00_00_00_00__00_00_00_02,
        //  JMP|K|EXIT
        0x95_00_00_00__00_00_00_00
    ];
    let p = unsafe { cbpf::Program::from_raw(&r[..]) };
    let c = cbpf::Invoke::new(p);
    assert_eq!(c.run(), 0x2);
}
