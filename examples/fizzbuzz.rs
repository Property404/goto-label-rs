#![allow(unused_assignments)]
use goto_label::{goto, label};

fn fizzbuzz() {
    let mut n = 1;

    unsafe {
        goto!("fizzbuzz_start");

        label!("print_fizzbuzz");
        println!("FizzBuzz");
        goto!("fizzbuzz_return");

        label!("print_fizz");
        println!("Fizz");
        goto!("fizzbuzz_return");

        label!("print_buzz");
        println!("Buzz");
        goto!("fizzbuzz_return");

        label!("fizzbuzz_start");

        if (n % 3 == 0) && (n % 5 == 0) {
            goto!("print_fizzbuzz");
        } else if n % 3 == 0 {
            goto!("print_fizz");
        } else if n % 5 == 0 {
            goto!("print_buzz");
        } else {
            println!("{n}");
        }

        label!("fizzbuzz_return");
        if n < 15 {
            n += 1;
            goto!("fizzbuzz_start");
        }
    }
}

fn main() {
    fizzbuzz();
}
