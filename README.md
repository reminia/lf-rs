# lf
![ci](https://github.com/reminia/lf-rs/actions/workflows/rust.yml/badge.svg)

lf is a command to locate file in the default file explorer of os, build & run it with cargo.
![demo](https://res.cloudinary.com/leecy-me/image/upload/v1711725389/open/lf_ph7jzy.gif)

## Command Usage
Clone and install it by `cargo install --path .`, then u can use lf command everywhere like:
```bash
lf file
```

## Library Usage
Open current directory and focus on src and target.

```rust
lf::open(&["src", "target"])
```
