# termbin

Termbin is a tool that lets you create [termbins](https://termbin.com/) (like pastebins) from the command line.

## Usage

```powershell
# create a termbin from a file
termbin main.rs

# can read from stdin as well, just omit the filename:
echo testing | termbin

# copy the url after creating it (instead of printing it)
termbin main.rs -c
```

## Installation

You'll need an up to date rust toolchain and git.

```sh
# recommended:
git clone https://github.com/Eshanatnight/termbin
cd termbin
cargo build --release

# alternative:
cargo install --git https://github.com/Eshanatnight/termbin --branch main
```
