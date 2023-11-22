# Hex Dump
Hex Dump is a minimalist and dummy hexadecimal viewer.

* **Install**

```
$ cargo install --git http://github.com/jgardona/hd
```

* **Usage**

```
$ hd --help

A minimalist hex viewer

Usage: hd [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Target file

Options:
  -s, --skip <SKIP>      Skip `n` offset(1 = 16) bytes [default: 0]
  -l, --length <LENGTH>  Only read `n` offset bytes from the input. Skip for full read
  -h, --help             Print help
  -V, --version          Print version
```

**Read 3 lines of 16 offset bytes**

```
$ hd -l 3 tests/data/data2
[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 
[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 
[0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 
```