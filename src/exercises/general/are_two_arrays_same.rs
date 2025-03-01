#![allow(dead_code)]

pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut b_processed_indexes: Vec<Option<usize>> = vec![];

    for a_value in a {
        let b_square_index = b.iter().enumerate().position(|(index, x)| {
            x == &(a_value * a_value) && !b_processed_indexes.contains(&Some(index))
        });

        if b_square_index.is_none() {
            return false;
        }

        b_processed_indexes.push(b_square_index);
    }

    b_processed_indexes.len() == b.len()
}

#[test]
fn tests_comp() {
    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    let a1 = vec![11];
    let a2 = vec![11 * 11, 12 * 12];
    testing(a1, a2, false);

    let a1 = vec![14, 12];
    let a2 = vec![12 * 12, 14 * 14];
    testing(a1, a2, true);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, -11];
    let a2 = vec![
        11 * 11,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, true);

    let a1 = vec![121, 144, -19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * -11,
        121 * 121,
        144 * 144,
        -19 * -19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, false);

    let a1 = vec![121, 144, -19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        -19 * -19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, false);
}
