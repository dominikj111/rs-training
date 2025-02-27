/*
 * The Fibonacci numbers are the numbers in the following integer sequence (Fn):
 * 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...
 *
 * such as
 * F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.
 *
 * Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying
 * F(n) * F(n+1) = prod.
 *
 * Some Examples of Return:
 *
 * productFib(714)
 * should return (21, 34, true), since F(8) = 21, F(9) = 34 and 714 = 21 * 34
 *
 * productFib(800)
 * should return (34, 55, false), since F(8) = 21, F(9) = 34, F(10) = 55 and 21 * 34 < 800 < 34 * 55
 *
 */

/* NOTE: Fibonacci structure which supports multiple threads access

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
*/

/* NOTE: Fibonacci structure which supports multiple threads access by using pre-computed all fibonacci numbers at compile-time
    The overhead is just an additional 1KB of "code".
*/

// we can make the code faster by finding all fibonacci number at compile time as there are only u64 for example.