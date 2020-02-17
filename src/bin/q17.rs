fn main() {
    let n = 30;
    dbg!(fib(n + 2));
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
