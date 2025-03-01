#![allow(dead_code)]

pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    word.chars()
        .map(|c| {
            if word.matches(c).count() == 1 {
                '('
            } else {
                ')'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
