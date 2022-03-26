use goto_label::{goto, label};

fn main() {
    unsafe {
        goto!("label0");
    }

    println!("This will be skipped");

    unsafe {
        label!("label0");
    }

    println!("Hello world!");
}
