fn cut(length: u32, member: u32, current_count: u32) -> u32 {
    match current_count {
        c if c >= length => 0,
        c if c <= member => {
            1 + cut(length, member, current_count * 2)
        },
        _ => {
            1 + cut(length, member, current_count + member)
        },
    }
}

fn main() {
    // println!("{}", cut(20, 3, 1));
    println!("{}", cut(100000, 5, 1));
}
