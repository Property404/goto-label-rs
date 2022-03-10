# Goto/Label for Rust

Tired of using newfangled control flow mechnisms like "loop," "while,"
and "for"?

Well worry no more! Finally, usable "goto" and "label" macros have arrived
for Rust! And they're `#![no_std]`!

```rust
use goto_label::{goto, label};

#[no_mangle] // Needed to prevent foo() from being optimized away
unsafe fn foo() {
    println!("This text will never be printed!");

    label!("label1");
    print!("Hello");
    goto!("label2");

    println!("Neither will this be printed!");
}

unsafe fn hello_world() {
    goto!("label1");
    println!("This won't be printed either!");

    label!("label2");
    println!(" World!");
}

unsafe {
    hello_world();
}
```

## Known issues

* Segfaults will occur when optimization is enabled, even in trivial examples
* Segfaults will also occur in debug mode on macOS

## Currently supported Architectures

* amd64/x86\_64
* aarch64

## See also

[Another goto implementation](https://github.com/clucompany/Goto)
