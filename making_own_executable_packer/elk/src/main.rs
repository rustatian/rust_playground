use std::{env, error::Error, fs};
use delf::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().skip(1).next().expect("usage: elk FILE");
    let input = fs::read(&input_path)?;

    println!("Analyzing {:?}...", input_path);

    let file = match delf::File::parse_or_print_error(input.as_ref()) {
        None => std::process::exit(1),
        Some(file) => {
            file
        },
    };

    println!("{:#?}", file);

    println!("Executing {:?}", input_path);
    use std::process::Command;
    let status = Command::new(input_path).status()?;
    if !status.success() {
        return Err("process did not exit successfully".into());
    }

    Ok(())
}
