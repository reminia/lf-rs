# lf
lf is a command to locate file in the default file explorer, build & run it with cargo.

## Command Usage
Clone and install it by `cargo install --path .`, then u can use lf command everywhere like:
```bash
lf somefile
```

## Library Usage
Open current directory and focus on src.

```rust
lf::open(&["src"])
```
