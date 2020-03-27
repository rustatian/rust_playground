mod parse;

// &'a [u8] is self.0
pub struct HexDump<'a>(&'a [u8]);

use std::fmt;
use std::fmt::Formatter;

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
    pub r#type: Type,
    pub machine: Machine,
}

impl File {
    const ELF_MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            bytes::complete::{tag, take},
            error::context,
            sequence::tuple,
            combinator::map,
            number::complete::le_u16,
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

        let (i, (r#type, machine)) = tuple((
            context("Type", map(le_u16, Type::from_u16)),
            context("Machine", map(le_u16, Machine::from_u16)),
        ))(i)?;

        let t = r#type;
        let m = machine;

        let res = Self { machine: m.unwrap(), r#type: t.unwrap() };
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

impl Machine {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        let original_i = i;
        use nom::{
            error::{ErrorKind, ParseError, VerboseError},
            number::complete::le_u16,
            Err,
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

        // SOLUTION 2
        // Returns Result<O2, E2>, where O2 is output of our mapping func
        // and E2 is the error
        map_res(le_u16, |x| match Self::from_u16(x) {
            Some(x) => Ok(x),
            None => Err(ErrorKind::Alt),
        })(i)
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


































