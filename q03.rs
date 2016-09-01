
#[derive(Debug)]
struct Card {
    id: usize,
    open: bool,
}
fn main() {
    let n = 101;
    let mut cards = prepare_cards(n);

    for i in 1..n {
        let mut target = i;
        while target < cards.len() {
            cards[target].open = !cards[target].open;
            target = target + i + 1;
        }
        println!("---------------------------");
        println!("{}人目", i);
        let ids = cards.iter().filter(|x| !x.open).map(|x| x.id).collect::<Vec<usize>>();
        println!("{:?}", ids);
    }
}

fn prepare_cards(n: usize) -> Vec<Card> {
    let mut cards = Vec::new();
    for i in 0..n {
        cards.push(Card{id: i + 1, open: false})
    }
    cards
}
