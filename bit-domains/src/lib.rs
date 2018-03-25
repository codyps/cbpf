#![no_std]

///
/// 
/// # "Znum"
///
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///    https://www-apr.lip6.fr/~mine/publi/article-mine-wing12.pdf
///
///  - http://research.cs.wisc.edu/wpis/papers/vmcai17.pdf
///  - http://bitmath.blogspot.com/2013/08/addition-in-bitfield-domain.html
///  - http://bitmath.blogspot.com/2014/02/addition-in-bitfield-domain-alternative.html
///  - "Abstract Domains for Bit-Level Machine Integer and Floating-point Operations"
///  - https://www.omnimaga.org/other-computer-languages-help/addition-in-the-bitfield-domain/

pub mod znum;
pub use znum::Znum;

pub mod tnum;
pub use tnum::Tnum;

pub mod rnum;
pub use rnum::Rnum;
