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
