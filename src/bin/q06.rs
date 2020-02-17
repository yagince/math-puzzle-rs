fn even(i: u64) -> u64 {
    i / 2
}
fn odd(i: u64) -> u64 {
    i * 3 + 1
}

fn corats(i: u64, init: u64) -> u64 {
    let next = match i % 2 {
        0 => even(i),
        _ => odd(i),
    };

    if next == 1 || next == init {
        next
    } else {
        corats(next, init)
    }
}

fn main() {
    let mut nums = Vec::new();

    for i in 2..10000 {
        if i == corats(odd(i), i) {
            nums.push(i);
        }
    }

    println!("{:?}", nums);
    println!("count: {}", nums.len());
}
