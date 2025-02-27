pub fn narcissistic(num: u64) -> bool {
    num == num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .map(|x| x.pow(num.to_string().len() as u32))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(
            actual, expected,
            "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}",
            input
        )
    }

    #[test]
    fn basic_tests() {
        dotest(7, true);
        dotest(371, true);
        dotest(122, false);
        dotest(4887, false);
        dotest(1000000000006, false);
    }
}
