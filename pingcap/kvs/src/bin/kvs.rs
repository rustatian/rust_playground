use clap::Clap;
use std::process::exit;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Valery Piashchynski")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Get(Get),
    Set(Set),
    Rm(Rm),
}

#[derive(Clap)]
struct Get {
    get: String
}

#[derive(Clap)]
struct Set {
    key: String,
    value: String,
}

#[derive(Clap)]
struct Rm {
    key: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Get(t) => {
            eprintln!("unimplemented");
            exit(1);
        }
        SubCommand::Set(s) => {
            eprintln!("unimplemented");
            exit(1);
        }
        SubCommand::Rm(r) => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}