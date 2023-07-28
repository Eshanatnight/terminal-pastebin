mod cmd;

use clap::Parser;
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(name = "termbin")]
#[command(author, version, about = "Create a termbin from the command line.", long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long = "clip")]
    clip: Option<bool>,

    #[arg(short = 'r', long = "remote")]
    remote: Option<String>,

    #[arg(short = 'f', long = "file")]
    file: OsString,
}

fn main() {
    let app = Cli::parse();
    let cmd = cmd::Cmd::from_args(
        app.file.to_str().unwrap(),
        app.clip.unwrap_or(false),
        app.remote.as_deref().unwrap_or("termbin.com:9999"),
    );
    if let Err(e) = cmd.run() {
        eprintln!("error: {}", e);
        std::process::exit(2);
    }
}
