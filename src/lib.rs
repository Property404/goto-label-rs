#![no_std]
//! `goto` and `label` for Rust!
//! Now you never have to use `while`, `for`, or `loop` again!
//!
//! # Example
//! ```
//! use goto_label::{goto, label, might_skip};
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
//!     might_skip!{println!("This won't be printed either!")};
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
    ($label:literal) => {
        $crate::might_skip! {{
            #[allow(named_asm_labels)]
            #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
            {
                use core::arch::asm;
                asm!(concat!($label, ":"));
            }
            #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
            compile_error!("`label!` not implemented for this architecture!");
        }}
    };
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
    ($label:literal) => {
        $crate::might_skip! {{
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
        }}
    };
}

/// Inform the compiler that this expression might be skipped by `goto!`
///
/// This prevents segfaults in optimized builds by preventing optimization with surrounding code
#[macro_export]
macro_rules! might_skip {
    ($expression:expr) => {{
        #[allow(unused_unsafe)]
        if unsafe {
            let x = 42;
            let y = &x as *const i32;
            // This will always be true but the compiler doesn't know that!
            core::ptr::read_volatile(y) == core::ptr::read_volatile(y)
        } {
            $expression
        } else {
            Default::default()
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        unsafe {
            let mut x = 0;
            goto!("end0");
            might_skip! {x = 42};
            label!("end0");
            assert_eq!(x, 0);
        }
    }
}
