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
    println!(" World!")
}

fn main() {
    unsafe {
        hello_world();
    }
}
