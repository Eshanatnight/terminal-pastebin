# termbin

Termbin is a tool that lets you create [termbins](https://termbin.com/) (like pastebins) from the command line.

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
