use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut n = 2;

    loop {
        let square = (2..=((n * 2) as f64).sqrt().floor() as u32)
            .into_iter()
            .map(|i| (i as u32).pow(2))
            .collect_vec();

        let list = (1..=n)
            .into_iter()
            .map(|i| {
                (
                    i as usize,
                    square
                        .iter()
                        .flat_map(|s| if s > &i { Some((s - i) as usize) } else { None })
                        .filter(|x| x <= &(n as usize) )
                        .collect_vec(),
                )
            })
            .collect();

        let mut used = vec![true];
        used.resize(n as usize, false);

        let result = check(
            1,
            &mut used,
            &list,
        );

        if !result.is_empty() {
            dbg!(n);
            dbg!(result);
            break;
        }
        n += 1;
    }
}

fn check(last: usize, used: &mut Vec<bool>, list: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if used.iter().all(|x| *x) && list[&1].contains(&last) {
        return vec![1];
    }

    if let Some(a) = list.get(&last) {
        for i in a {
            if !used[i - 1] {
                used[i - 1] = true;
                let result = check(*i, used, list);
                if !result.is_empty() {
                    return vec![vec![*i], result].concat();
                }
                used[i - 1] = false;
            }
        }
    }
    vec![]
}
