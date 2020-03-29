mod parse;

use std::fmt;
use std::fmt::{Formatter, Debug};
use derive_more::*;

// &'a [u8] is self.0
pub struct HexDump<'a>(&'a [u8]);

impl<'a> fmt::Debug for HexDump<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // take only first 20 numbers
        for &x in self.0.iter().take(20) {
            write!(f, "{:02x} ", x)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct File {
    // offset: 0x10 2-bytes e_type
    pub r#type: Type,
    // offset: 0x12 2-bytes e_machine
    pub machine: Machine,
    // offset: 0x18 8-bytes e_entry
    pub entry_point: Addr,

}

impl File {
    const ELF_MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            combinator::verify,
            number::complete::le_u32,
            bytes::complete::{tag, take},
            error::context,
            sequence::tuple,
        };
        // 7f45 4c46 0201 0100 0000 0000 0000 0000
        let (i, _) = tuple((
            context("Magic", tag(Self::ELF_MAGIC)),
            context("Class", tag(&[0x2])),
            context("Endianness", tag(&[0x1])),
            context("Version", tag(&[0x1])),
            context("OS ABI", tag(&[0x0])),
            context("Padding", take(8_usize)),
        ))(i)?;

        let (i, (r#type, machine)) = tuple((Type::parse, Machine::parse))(i)?;

        // offset: 0x14 4-bytes e_version
        let (i, _) = context("Version (bis)", verify(le_u32, |&x| x == 1))(i)?;

        let (i, entry_point) = Addr::parse(i)?;

        let res = Self { machine, r#type, entry_point };
        Ok((i, res))
    }

    #[allow(clippy::match_wild_err_arm)]
    pub fn parse_or_print_error(i: parse::Input) -> Option<Self> {
        match Self::parse(i) {
            Ok((_, file)) => {
                Some(file)
            }
            Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => {
                eprintln!("Parsing failed: ");
                for (input, err) in err.errors {
                    use nom::Offset;
                    let offset = i.offset(input);
                    eprintln!("{:?} at position {}:", err, offset);
                    eprintln!("{:>08x}: {:?}", offset, HexDump(input));
                }
                None
            }
            Err(_) => panic!("unexpected nom error"),
        }
    }
}

// Addr is 8-bytes (64 bit platform) long
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Add, Sub)]
pub struct Addr(pub u64); // tuple struct with 1 element, NEWTYPE pattern

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Into<u64> for Addr {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for Addr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<u64> for Addr {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl Addr {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{combinator::map, number::complete::le_u64};
        map(le_u64, Addr::from)(i)
    }
}

// e-type, 16 byte
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum Type {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

impl Type {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            error::{context, ErrorKind},
            number::complete::le_u16,
            combinator::map_res,
        };

        context("Type", map_res(le_u16, |x| Type::from_u16(x).ok_or(ErrorKind::Alt)))(i)
    }

    pub fn from_u16(x: u16) -> Option<Self> {
        match x {
            0 => Some(Self::None),
            1 => Some(Self::Rel),
            2 => Some(Self::Exec),
            3 => Some(Self::Dyn),
            4 => Some(Self::Core),
            _ => None,
        }
    }
}

// e-machine
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum Machine {
    X86 = 0x03,
    X86_64 = 0x3e,
}

// impl_parse_for_enum!(Type, le_u16);
// impl_parse_for_enum!(Machine, le_u16);

impl Machine {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            error::{context, ErrorKind},
            number::complete::le_u16,
            combinator::map_res,
        };

        // SOLUTION 1

        // let (i, x) = le_u16(i)?;
        // match Self::from_u16(x) {
        //     None => {
        //         Err(Err::Failure(VerboseError::from_error_kind(
        //             original_i,
        //             ErrorKind::Alt,)))
        //     },
        //     Some(res) => {
        //         Ok((i, res))
        //     },
        // }

        // SOLUTION 2 anti-pattern, option<t> convert to the Result<T,E>

        // Returns Result<O2, E2>, where O2 is output of our mapping func
        // and E2 is the error
        // map_res(le_u16, |x| match Self::from_u16(x) {
        //     Some(x) => Ok(x),
        //     None => Err(ErrorKind::Alt),
        // })(i)

        // SOLUTION 3
        context("Machine", map_res(le_u16, |x| Self::from_u16(x).ok_or(ErrorKind::Alt)))(i)
    }
    pub fn from_u16(m: u16) -> Option<Self> {
        match m {
            0x03 => Some(Self::X86),
            0x3e => Some(Self::X86_64),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn type_to_u16() {
        assert_eq!(super::Type::Dyn as u16, 0x3)
    }

    #[test]
    fn type_from_u16() {
        assert_eq!(super::Type::from_u16(0x3), Some(super::Type::Dyn));
        assert_eq!(super::Type::from_u16(0xf00d), None)
    }

    fn machine_from_u16() {
        assert_eq!(super::Machine::X86_64 as u16, 0x3e);
        assert_eq!(super::Machine::from_u16(0x3e), Some(super::Machine::X86_64));
        assert_eq!(super::Machine::from_u16(0xfa), None)
    }
}


































