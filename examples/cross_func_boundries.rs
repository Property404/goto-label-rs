use goto_label::{goto, label, might_skip};

#[no_mangle] // Needed to prevent foo() from being optimized away
unsafe fn foo() {
    might_skip! {println!("This text will never be printed!")}

    label!("label1");
    might_skip! {print!("Hello")}
    goto!("label2");

    might_skip! {println!("Neither will this be printed!")}
}

unsafe fn hello_world() {
    goto!("label1");
    might_skip! {println!("This won't be printed either!")}

    label!("label2");
    println!(" World!")
}

fn main() {
    unsafe {
        hello_world();
    }
}
