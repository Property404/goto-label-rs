#![no_std]
//! `goto` and `label` for Rust!
//! Now you never have to use `while`, `for`, or `loop` again!
//!
//! # Example
//! ```
//! use goto_label::{goto, label};
//!
//! #[no_mangle] // Needed to prevent foo() from being optimized away
//! unsafe fn foo() {
//!     println!("This text will never be printed!");
//!
//!     label!("label1");
//!     print!("Hello");
//!     goto!("label2");
//!
//!     println!("Neither will this be printed!");
//! }
//!
//! unsafe fn hello_world() {
//!     goto!("label1");
//!     println!("This won't be printed either!");
//!
//!     label!("label2");
//!     println!(" World!");
//! }
//!
//! unsafe {
//!     hello_world();
//! }
//! ```

/// Create a label
///
/// This will create a linker symbol. Be careful that the label you use does not clash with other
/// symbols.
///
/// # Example
/// ```
/// use goto_label::label;
///
/// // Create a label named "foo"
/// unsafe {
///     label!("foo");
/// }
/// ```
#[macro_export]
macro_rules! label {
    ($label:literal) => {{
        #[allow(named_asm_labels)]
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        {
            use core::arch::asm;
            asm!(concat!($label, ":"));
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        compile_error!("`label!` not implemented for this architecture!");
    }};
}

/// Jump to label
///
/// # Example
/// ```
/// use goto_label::{goto, label};
///
/// unsafe {
///     // Jump to label named "foo"
///     goto!("foo");
///     println!("This line will never be printed!");
///
///     // Label is defined here
///     label!("foo");
/// }
/// ```
#[macro_export]
macro_rules! goto {
    ($label:literal) => {{
        use core::arch::asm;
        #[allow(named_asm_labels)]
        #[cfg(any(target_arch = "x86_64"))]
        {
            asm!(concat!("jmp ", $label));
        }
        #[cfg(any(target_arch = "aarch64"))]
        {
            asm!(concat!("b ", $label));
        }
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        compile_error!("`goto!` not implemented for this architecture!");
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn count() {
        const MAX: i32 = 10;
        let mut x = 0;
        unsafe {
            label!("start_0");
            x += 1;
            if x < MAX {
                goto!("start_0")
            }
        }
        assert_eq!(x, MAX);
    }
}
