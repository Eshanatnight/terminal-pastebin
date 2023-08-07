# termbin

Termbin is a tool that lets you create [termbins](https://termbin.com/) (like pastebins) from the command line.

## Elevator Pitch

Does your workplace, anti-virus block tools like `netcat`? And send code snippets over Google Chat 😀. And refuse to use 
useful tools like `telnet`? And makes you write code on `vscode` 🙅‍♂️?
Then this is the tool for you! just take the code file and send it to the cli and boom 👾, a link appears magically 🧙 in the terminal.

## Usage

```bash
# create a termbin from a file
termbin --file main.rs

# copy the url after creating it (instead of printing it)
termbin main.rs -c true
```

## Building

You'll need an up to date rust toolchain and git.

```bash
git clone https://github.com/Eshanatnight/termbin
cd termbin
cargo build --release
```

## Thanks

A huge thanks to the amazing people at [Termbin](https://termbin.com).
