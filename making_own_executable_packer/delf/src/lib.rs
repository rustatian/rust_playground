mod parse;

#[derive(Debug)]
pub struct File {}

impl File {
    const ELF_MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
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

        Ok((i, Self {}))
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
    Loos = 0xfe00,
    Hios = 0xfeff,
    Loproc = 0xff00,
    Hiproc = 0xffff,
}

impl Type {
    pub fn from_u16(x: u16) -> Option<Self> {
        match x {
            0 => Some(Self::None),
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_to_u16() {
        assert_eq!(super::Type::Dyn as u16, 0x3)
    }
}
