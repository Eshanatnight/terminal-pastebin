mod cmd;

use clap::Parser;
use cmd::Cmd;
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(name = "termbin")]
#[command(author, version, about = "Create a termbin from the command line.", long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long = "clip")]
    clip: Option<bool>, // try with bool rather than using Option<bool> https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/03_01_flag_bool.rs

    #[arg(short = 'r', long = "remote")]
    remote: Option<String>, // #[clap(default_value_t=termbin.com:9999)]

    #[arg(index = 1)]
    file: OsString,
}

fn main() {
    let app: Cli = Cli::parse();
    let cmd: Cmd = Cmd::from_args(
        app.file.to_str().unwrap(),
        app.clip.unwrap_or(false),
        app.remote.as_deref().unwrap_or("termbin.com:9999"),
    );
    if let Err(e) = cmd.run() {
        eprintln!("error: {}", e);
        std::process::exit(2);
    }
}
