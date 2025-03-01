mod exercises;

use exercises::general::prelude::*;

mod exercises_general {
    pub use super::exercises::general::narcissistic_numbers::narcissistic;
}

fn main() {
    println!("{:?}", two_sum(&[2, 2, 3, 2, 5], 4)); // (0, 1)
    println!("narcissistic: {}", exercises_general::narcissistic(153)); // true
    println!("Hello, World!")
}
