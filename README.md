# EasyTerm

## Cargo.toml

```toml
[dependencies]
easyterm = "0.1.0"
```

## How to use

and add this to your `lib.rs` or `main.rs`:

```rust
    use easyterm::EasyTerm;

    fn main() {
        EasyTerm::set_cursor_invisible();
        EasyTerm::clear_screen();
        EasyTerm::set_cursor_top_left();
        println!("Text at top left corner in absolutely clear terminal");
        EastTerm::move_cursor_down_by(n: 2);
        println!("Text 2 lines below")
    }
```