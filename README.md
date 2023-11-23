# MHV
**mhv** is a minimalist hexadecimal viewer.

<div align="center">
<img src="images/demo.png"/>
</div>


* **Install**

```
$ cargo install mhv
```

* **Usage**

```
$ mhv --help

A minimalist hex viewer

Usage: mhv [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Target file

Options:
  -s, --skip <SKIP>      Skip `n` bytes [default: 0]
  -l, --length <LENGTH>  Read `n` bytes. None for full read
  -h, --help             Print help
  -V, --version          Print version
```

* **Read 5 bytes from start**

![](images/read5.png)


* **Read 32 bytes from start**

![](images/read32fromstart.png)

* **Skip 16 bytes and read 40**

![](images/skip16andread40.png)
