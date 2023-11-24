# MHV

<div align="center">
<img src="images/demo.png"/>
</div>

## **MHV** is a minimalist hexadecimal viewer.

* **The color legend**
  
  *  <span style="color: gray;">**Null byte**</span>
  *  <span style="color: lightblue;">**ASCII Printable Characters**</span>
  *  <span style="color: lightgreen;">**Space Characters**</span>
  *  <span style="color: lightgreen;">**Control Characters**</span>
  *  <span style="color: orangered;">**ASCII Extended Codes**</span>
  


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
