// Maintained by ./tools/update_mods.sh

#![allow(unused_imports)]

pub mod are_two_arrays_same;
pub mod break_camelcase;
pub mod duplicate_encode;
pub mod good_vs_evil;
pub mod narcissistic_numbers;
pub mod product_fib;
pub mod two_sum;

pub mod prelude {
    pub use super::are_two_arrays_same::comp as are_two_arrays_same;
    pub use super::break_camelcase::break_camelcase;
    pub use super::duplicate_encode::duplicate_encode;
    pub use super::good_vs_evil::good_vs_evil;
    pub use super::narcissistic_numbers::narcissistic as narcissistic_numbers;
    pub use super::product_fib::product_fib;
    pub use super::two_sum::two_sum;
}
