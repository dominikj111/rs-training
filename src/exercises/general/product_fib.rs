#![allow(dead_code)]

struct Fibonacci {
    values: [usize; 128],
}

impl Fibonacci {
    const fn new() -> Self {
        let mut length = 2;
        let mut values = [0usize; 128];

        values[1] = 1;

        while let Some(sum) = values[length - 2].checked_add(values[length - 1]) {
            values[length] = sum;
            length += 1;
        }

        Self { values }
    }

    fn get_or_compute(&self, index: usize) -> usize {
        self.values[index]
    }
}

static FIBONACCI: Fibonacci = Fibonacci::new();

fn fib_get(index: usize) -> usize {
    FIBONACCI.get_or_compute(index)
}

pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fibonacci_pointer: usize = 0;

    loop {
        let f1 = fib_get(fibonacci_pointer);
        let f2 = fib_get(fibonacci_pointer + 1);
        let f1f2 = (f1 as u64) * (f2 as u64);

        if f1f2 == prod {
            return (f1 as u64, f2 as u64, true);
        }

        if f1f2 > prod {
            return (f1 as u64, f2 as u64, false);
        }

        fibonacci_pointer += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{fib_get, product_fib};

    #[test]
    fn fibonacci_struct_api() {
        assert_eq!(fib_get(0), 0);
        assert_eq!(fib_get(1), 1);
        assert_eq!(fib_get(2), 1);
        assert_eq!(fib_get(3), 2);
        assert_eq!(fib_get(4), 3);
        assert_eq!(fib_get(5), 5);
        assert_eq!(fib_get(6), 8);
        assert_eq!(fib_get(7), 13);
        assert_eq!(fib_get(19), 4181);
        assert_eq!(fib_get(15), 610);
        assert_eq!(fib_get(11), 89);
    }

    #[test]
    fn basics_product_fib() {
        #[track_caller]
        fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
            assert_eq!(product_fib(prod), exp)
        }

        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
