# Product of Fibonacci Numbers

The Fibonacci numbers are the numbers in the following integer sequence (Fn):
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...

Such as:
F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.

## Problem Description

Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying:
F(n) \* F(n+1) = prod.

Your function should return a tuple or array with F(n), F(n+1), and a boolean value indicating whether the product F(n) \* F(n+1) equals the given number.

## Examples

```rust
product_fib(714) // should return (21, 34, true)
// since F(8) = 21, F(9) = 34 and 714 = 21 * 34

product_fib(800) // should return (34, 55, false)
// since F(8) = 21, F(9) = 34, F(10) = 55 and 21 * 34 < 800 < 34 * 55
```

## Function Signature

```rust
fn product_fib(prod: u64) -> (u64, u64, bool)
```

## Implementation Notes

There are several approaches to implementing the Fibonacci sequence calculation:

### 1. Simple Iterative Approach

```rust
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;

    while a * b < prod {
        let c = a + b;
        a = b;
        b = c;
    }

    (a, b, a * b == prod)
}
```

### 2. Pre-computed Fibonacci Numbers

For better performance, you could pre-compute Fibonacci numbers and store them in a structure:

```rust
struct Fibonacci {
    values: [usize; 128],
}

impl Fibonacci {
    fn new() -> Self {
        let mut fib = Self { values: [0; 128] };
        fib.values[0] = 0;
        fib.values[1] = 1;

        for i in 2..128 {
            fib.values[i] = fib.values[i-1] + fib.values[i-2];
        }

        fib
    }
}
```

### 3. Thread-Safe Implementation

For a more advanced implementation that supports multiple threads:

```rust
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct Fibonacci {
    values: Vec<AtomicUsize>,
}

impl Fibonacci {
    const fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn get_or_compute(&mut self, index: usize) -> usize {
        if self.values.is_empty() {
            self.values.push(AtomicUsize::new(0));
            self.values.push(AtomicUsize::new(1));
        }

        while self.values.len() - 1 < index {
            self.values.push(AtomicUsize::new(
                self.values[self.values.len() - 1].load(Ordering::Relaxed)
                    + self.values[self.values.len() - 2].load(Ordering::Relaxed),
            ));
        }

        self.values[index].load(Ordering::Relaxed)
    }
}

use std::sync::Mutex;

static FIBONACCI: Mutex<Fibonacci> = Mutex::new(Fibonacci::new());

fn fib_get(index: usize) -> usize {
    FIBONACCI.lock().unwrap().get_or_compute(index)
}
```

## Test Cases

- `product_fib(714)` should return `(21, 34, true)`
- `product_fib(800)` should return `(34, 55, false)`
- `product_fib(4895)` should return `(55, 89, true)`
- `product_fib(5895)` should return `(89, 144, false)`
