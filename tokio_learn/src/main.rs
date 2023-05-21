use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let (mut reader, mut writer) = os_pipe::pipe()?;
    writer.write_all(b"x")?;
    let mut output = [0];
    reader.read_exact(&mut output)?;
    assert_eq!(b"x", &output);

    Ok(())
}
