
fn main() {
    let mut i = 10;

    loop {
        let decimal = format!("{}", i);
        let binary  = format!("{:b}", i);
        let octa    = format!("{:o}", i);

        if is_circular(&decimal) && is_circular(&binary) && is_circular(&octa) {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

fn is_circular(v: &String) -> bool {
    *v == v.chars().rev().collect::<String>()
}
