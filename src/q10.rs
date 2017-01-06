
fn main() {
    let europian = vec![0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5,
                        24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26];
    let american = vec![0, 28, 9, 26, 30, 11, 7, 20, 32, 17, 5, 22, 34, 15, 3, 24, 36, 13, 1, 0,
                        27, 10, 25, 29, 12, 8, 19, 31, 18, 6, 21, 33, 16, 4, 23, 35, 14, 2];
    let count = (2..36)
        .map(|i| sum_max(&europian, i) < sum_max(&american, i))
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len();
    println!("答えは {} 個", count);
}

fn sum_max(nums: &Vec<u32>, n: usize) -> u32 {
    let ans = nums.iter().take(n).fold(0, |acc, &i| acc + i);

    let (ans, _) = (0..(nums.len() - 1)).fold((ans, ans), |(acc, prev_total), i| {
        let total = prev_total + nums[(i + n) % nums.len()] - nums[i];
        (vec![total, acc].into_iter().max().unwrap(), total)
    });
    ans
}
