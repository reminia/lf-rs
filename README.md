# lf
lf is a command to locate file in the default file explorer.

## Run
Try it with `cargo run file`, it will open current directory and focus on the file.

## Command Usage
Clone and install it by `cargo install --path .`, then u can use ls command everywhere like:
```bash
lf somefile
```

## Library Usage
Open current directory and focus on src.

```rust
lf::open(&["src"])
```
