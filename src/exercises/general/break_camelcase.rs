pub fn break_camelcase(words: &str) -> String {
    let mut new_string = String::new();

    for c in words.chars() {
        if c.is_uppercase() {
            new_string.push(' ');
            new_string.push(c);
        } else {
            new_string.push(c);
        }
    }

    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(break_camelcase("camelCasing"), "camel Casing");
        assert_eq!(break_camelcase("camelCasingTest"), "camel Casing Test");
    }
}
