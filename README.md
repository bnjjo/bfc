# bfc
Naive implementation of a brainfuck to x86-64 GNU assembly compiler.
## Installation
> [!IMPORTANT]
> Please note that this is only compatible with Linux systems using an AMD64 processor.<br />
> **GCC is also required to assemble and link the compiled output!**

Install GCC if you haven't already:

For Debian-based distros:

```bash
sudo apt install gcc
```

For Arch-based distros:

```bash
sudo pacman -S gcc
```

Once you have done that:
1. Download the precompiled binary in the [Releases page](https://github.com/bnjjo/bfc/releases)
2. Mark it as executable:
```bash
chmod +x bfc
```
3. (Optionally) move it to your PATH:
```bash
sudo mv bfc /usr/local/bin  # system-wide
mv bfc ~/.local/bin         # per-user
```

## Usage
```bash
bfc input.bf output
./output
```
or in one line:
```bash
bfc input.bf output && ./output
```

## Building from source
```bash
git clone https://github.com/bnjjo/bfc.git
cd bfc
cargo build --release
./target/release/bfc input.bf output
```

## Examples & credit
I have included a few fun brainfuck programs in the
[*examples* directory of this repo.](https://github.com/bnjjo/bfc/tree/master/examples) Some of which are:
* **ascii.bf** – Prints all 256 standard + extended ASCII characters
* **btc.bf** by Katie Ball – A brainfuck to C interpreter
* **cat.bf** – A simple cat program, copies stdin to stdout (i.e. writes out whatever you input)
* **hanoi.bf** by Clifford Wolf – A towers of Hanoi problem simulation
* **helloworld.bf** by Katie Ball – You can guess what this one does lol
* **mandelbrot.bf** by Erik Bosman – A mandelbrot set fractal viewer

If you're interested in learning more about brainfuck check out this well-written
[tutorial](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a) by Katie Ball
(the person who made the brainfuck to C interpreter and helloworld program!)
