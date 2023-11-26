# MHV

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
$ mhv --help

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
  -n, --no-squeezing  Displays all input data. Otherwise any number of output
                      lines which would be identical to the last one are replaced
                      with a line comprised of a single asterisk.
  -h, --help          Print help
  -V, --version       Print version
```

## Usage examples

* **Read 5 bytes from start**

![](images/read5.png)


* **Read 32 bytes from start**

![](images/read32fromstart.png)

* **Skip 16 bytes and read 40**

![](images/skip16andread40.png)

* **Read using units (kb or mb)**

![](images/read1kb.png)
