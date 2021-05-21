<div align="center">
    <h1><b>Arch Mirrors</b></h1>
    <a href="https://www.crates.io/crates/arch-mirrors">
        <img src="https://img.shields.io/crates/v/arch-mirrors.svg">
    </a>
    <a href="https://www.docs.rs/arch-mirrors">
        <img src="https://docs.rs/arch-mirrors/badge.svg">
    </a>
    <p>Parse the Arch Linux mirror status</p>
</div>

# Description

The `arch-mirrors` crate allows you to parse the JSON from https://www.archlinux.org/mirrors/status/json into a typed,
rusty form. 

# Examples

For the examples see [examples](examples).

# Installation

With [`cargo-edit`](https://crates.io/crates/cargo-edit):

```bash
$ cargo add arch-mirrors
```

Or via `Cargo.toml`:

```toml
[dependencies]
arch-mirrors = "0.1.0"
```
