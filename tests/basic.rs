extern crate cbpf;

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
    let p = cbpf::Program::verify(&r[..]).unwrap();

    let c = cbpf::Context::new(p);

    assert_eq!(c.run(), 0x1);
}
