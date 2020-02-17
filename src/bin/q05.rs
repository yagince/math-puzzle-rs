// struct Money {
//     amount: i32,
// }

// fn change(orig: i32, coins: Vec<Money>, max_count: i32) -> Vec<Money> {
//     if coins.len() == 0 {
//         return vec![];
//     }

//     match coins.split_at(1) {
//         (Some(m), tail) => {
//             let tmp_coins = vec![head];

//             if tail.len() == 0 {
//                 return tmp_coins
//             } else {
//                 change(orig - head.amount, tail, max_count - 1)
//             }
//         },
//     }
// }

fn main() {
    let a: Vec<i32> = vec![1];
    match a.split_first() {
        Some((head, tail)) => println!("{:?}, {:?}", head, tail),
        None => println!("hoge"),
    }
}
