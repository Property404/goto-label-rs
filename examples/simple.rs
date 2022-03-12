use goto_label::{goto, label, might_skip};

fn main() {
    unsafe {
        goto!("label0");
    }
    might_skip! {println!("This will be skipped")};
    unsafe {
        label!("label0");
    }
    println!("Hello world!");
}
