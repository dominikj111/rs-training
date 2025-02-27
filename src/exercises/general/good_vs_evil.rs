const GOOD_BOYS: [u8; 6] = [
    1,  // Hobbits
    2,  // Men
    3,  // Elves
    3,  // Dwarves
    4,  // Eagles
    10, // Wizards
];

const BAD_BOYS: [u8; 7] = [
    1,  // Orcs
    2,  // Men
    2,  // Wargs
    2,  // Goblins
    3,  // Uruk Hai
    5,  // Trolls
    10, // Wizards
];

pub fn good_vs_evil(good: &str, evil: &str) -> String {
    let mut good_sum = 0;
    let mut evil_sum = 0;

    for (index, item) in good
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .enumerate()
    {
        good_sum += (GOOD_BOYS[index] as u32) * (item as u32);
    }

    for (index, item) in evil
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .enumerate()
    {
        evil_sum += (BAD_BOYS[index] as u32) * (item as u32);
    }

    if good_sum > evil_sum {
        return String::from("Battle Result: Good triumphs over Evil");
    }

    if good_sum < evil_sum {
        return String::from("Battle Result: Evil eradicates all trace of Good");
    }

    String::from("Battle Result: No victor on this battle field")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
            "Battle Result: Good triumphs over Evil"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
            "Battle Result: Evil eradicates all trace of Good"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
            "Battle Result: No victor on this battle field"
        );
    }
}
