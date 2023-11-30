# MHV
[![Crates.io](https://img.shields.io/crates/v/mhv.svg)](https://crates.io/crates/mhv)
[![Crates.io Downloads](https://img.shields.io/crates/d/mhv)](https://crates.io/crates/mhv)
![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License](https://img.shields.io/crates/l/mhv.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/jgardona/mhv/actions/workflows/rust.yml/badge.svg)](https://github.com/jgardona/mhv/actions/workflows/rust.yml)

<div align="center">
<img src="images/demo.png"/>
</div>

## **MHV** is a minimalist hexadecimal viewer.

* **The color legend**
  
  *  ⬛ **Null byte**
  *  🟦 **ASCII Printable Characters**
  *  🟩 **Space Characters**
  *  🟩 **Control Characters**
  *  🟥 **ASCII Extended Codes**
  


* **Install**

```
$ cargo install mhv
```

## Usage

```
❯ mhv -h
A minimalist hex viewer

Usage: mhv [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Target file

Options:
  -s, --skip <N>      Skip `N` bytes of the input. The `N` argument can also
                      include an unit (see `--length` for details). [default: 0]
  -l, --length <N>    Read `N` bytes from the input. None for full read. The `N`
                      argument can be a unit with a decimal prefix(kb, mb).
                      Examples: --length 3kb, -l3kb, --length 1mb...
                      N unis are kb(1000), K(1024), mb(1000 * 1000) M(1024 * 1024),
                      and a prefix 0x for hexadecimal, `0x0a`.
  -n, --no-squeezing  Displays all input data. Otherwise any number of output
                      lines which would be identical to the last one are replaced
                      with a line comprised of a single asterisk.
  -h, --help          Print help
  -V, --version       Print versio
```
## Stream Workflow

![](images/mhv-dataflow.png)

## Usage examples

* **Read 5 bytes from start**

![](images/read5.png)


* **Read 32 bytes from start**

![](images/read32fromstart.png)

* **Skip 16 bytes and read 40**

![](images/skip16andread40.png)

* **Read using units kb(1000), K(1024), mb(1000 * 1000) or M(1024 * 1024)**

![](images/read1kb.png)

* **Read from offset 0x50 plus 16 bytes**

![](images/off0x50-32bytes.png)

* **Read UTF16 file**

![](images/readutf16.png)

