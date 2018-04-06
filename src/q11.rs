use std::cell::UnsafeCell;
use std::collections::HashMap;

fn main() {
    let mut fib = Fib::new();

    let mut nums = Vec::new();
    let mut i = 3;
    loop {
        let num = fib.from_cache(i);
        let total = num.to_string().chars().map(|c| c.to_string().parse::<i64>().unwrap()).fold(0, |acc, i| acc + i);

        if num % total == 0 {
            nums.push(num.clone());
        }
        if nums.len() == 11 {
            break;
        }
        i += 1;
    }

    println!("{:?}", nums);
}

struct Fib {
    cache: UnsafeCell<HashMap<i64, i64>>,
}

impl Fib {
    pub fn new() -> Fib {
        Fib{
            cache: UnsafeCell::new(HashMap::new()),
        }
    }

    pub fn from_cache(&mut self, n: i64) -> i64 {
        unsafe {
            (*self.cache.get()).entry(n).or_insert_with(|| self.fib(n)).clone()
        }
    }

    fn fib(&mut self, n: i64) -> i64 {
        match n {
            0 | 1 => 1,
            _ => {
                self.from_cache(n - 2) + self.from_cache(n - 1)
            },
        }
    }
}
