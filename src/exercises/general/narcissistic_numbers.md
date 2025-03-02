# Narcissistic Numbers (Armstrong Numbers)

## Problem Description

A Narcissistic Number (or Armstrong Number) is a positive number which is the sum of its own digits, each raised to the power of the number of digits in a given base. In this exercise, we will restrict ourselves to decimal (base 10).

## Examples

- **153** (3 digits) is narcissistic:

  ```rust
  1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
  ```

- **1652** (4 digits) is not narcissistic:

  ```rust
  1^4 + 6^4 + 5^4 + 2^4 = 1 + 1296 + 625 + 16 = 1938
  ```

## Challenge

Write a function that returns `true` or `false` depending upon whether the given number is a Narcissistic number in base 10.

## Function Signature

```rust
fn narcissistic(num: u64) -> bool
```

## Implementation Notes

A possible implementation approach:

1. Convert the number to a string to count its digits
2. Extract each digit
3. Raise each digit to the power of the total number of digits
4. Sum the results
5. Check if the sum equals the original number

## Test Cases

- `narcissistic(7)` should return `true`
- `narcissistic(371)` should return `true`
- `narcissistic(122)` should return `false`
- `narcissistic(4887)` should return `false`

// posible implementation
<!-- markdownlint-disable-next-line MD033 -->
// let digits: Vec<u8> = num
// .to_string()
// .chars()
// .map(|c| c.to_digit(10).unwrap() as u8)
// .collect();

// let exponent = digits.len() as u32;
// let maybe_narcissistic = digits.iter().fold(0, |acc, &x| acc + (x as u128).pow(exponent) as u64);

// num == maybe_narcissistic
