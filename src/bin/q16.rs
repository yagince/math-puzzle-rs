use itertools::Itertools;
use num_rational::Ratio;
use std::collections::HashSet;

fn main() {
    answer_1(500);
    answer_2(500);
}

fn answer_1(max: u32) {
    dbg!(max);
    let a = (1..=(max / 4))
        .into_iter()
        .flat_map(|c| {
            (1..=(c - 1))
                .into_iter()
                .map(|x| x * (2 * c - x))
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    if a + b == c * c {
                        Some([(a, b).into(), ((c * c), a).into()])
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect::<HashSet<[Ratio<_>; 2]>>();
    dbg!(a.len());
}

fn answer_2(max: u32) {
    dbg!(max);
    let a = (1..=(max / 4))
        .into_iter()
        .flat_map(|c| {
            (1..=(c - 1))
                .into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    if a.pow(2) + b.pow(2) == c.pow(2) && num_integer::gcd(a, b) == 1 {
                        Some((a, b))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect::<HashSet<_>>();
    dbg!(a.len());
}
