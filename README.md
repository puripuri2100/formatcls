![](https://github.com/puripuri2100/formatcls/workflows/Rust/badge.svg)


# xml2saty-rust

This software converts xml file to SATySFi's document file.


# Install using Cargo

Here is a list of minimally required softwares.

* git
* make
* Rust


## Example

### Install Rust and cargo (Ubuntu)

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Install Rust and cargo (Ubuntu on WSL)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Rust and cargo (Windows)

Please download [installer](https://www.rust-lang.org/tools/install), and starting installer.

### Build and Install

```sh
git clone https://github.com/puripuri2100/formatcls.git
cd xml2saty-rust

make install
```


# Usage of xml2saty-rust

Type

```sh
formatcls -c <config file> -o <output file>
```

## Starting out

```sh
make demo
```

If `example/gengou.saty` and `example/keihou.saty` are created, then the setup has been finished correctly.

---

This software released under [the MIT license](https://github.com/puripuri2100/formatcls/blob/master/LICENSE).

Copyright (c) 2020 Naoki Kaneko (a.k.a. "puripuri2100")