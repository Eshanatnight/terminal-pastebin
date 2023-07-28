use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Cmd {
    file: String,
    clip: bool,
    remote: String,
}

impl Cmd {
    pub fn from_args(file_path: &str, clip: bool, remote: &str) -> Self {
        Self {
            file: file_path.to_string(),
            clip,
            remote: remote.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let url = self.send_file()?;

        if self.clip {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(self.file.clone()).map(|_| {
                println!("copied the link to clipboard");
            })
        } else {
            println!("{}", &url);
            Ok(())
        }
    }
}

impl Cmd {
    fn send_file(&self) -> Result<String, io::Error> {
        let mut file = File::open(&self.file)?;
        let mut stream = TcpStream::connect(&self.remote)?;
        io::copy(&mut file, &mut stream)?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        Ok(buf)
    }
}
