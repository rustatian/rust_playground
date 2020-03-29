pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

#[macro_export]
macro_rules! impl_parse_for_enum {
    ($type:ident, $number_parser:ident) => {
        impl $type {
            pub fn parse(i: parse::Input) -> parse::Result<Self> {
                use nom::{
                    error::{context, ErrorKind},
                    number::complete::le_u16,
                    combinator::map_res,
                };

                let parser = map_res($number_parser, |x| Self::from_u16(x).ok_or(ErrorKind::Alt));
                context(stringify!($type), parser)(i)
            }
        }
    };
}
