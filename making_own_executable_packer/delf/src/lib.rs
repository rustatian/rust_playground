use std::convert::TryFrom;

mod parse;

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
            context("Type", map(le_u16, |x| Type::from_u16(x))),
            context("Machine", map(le_u16, |x| Machine::from_u16(x),
            )),
        ))(i)?;

        let t = r#type;
        let m = machine;

        let res = Self { machine: m.unwrap(), r#type: t.unwrap() };
        Ok((i, res))
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


































