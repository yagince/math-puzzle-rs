fn main() {
    println!("-- Q14");

    let countries = [
        "Brazil",
        "Croatia",
        "Mexico",
        "Cameroon",
        "Spain",
        "Netherlands",
        "Chile",
        "Australlia",
        "Columbia",
        "Greece",
        "Cote d'lvoire",
        "Japan",
        "Uruguay",
        "Costa Rica",
        "England",
        "Italy",
        "Switzerland",
        "Ecuador",
        "France",
        "Honduras",
        "Argentina",
        "Bosnia and Herzegovina",
        "Iran",
        "Nigeria",
        "Germany",
        "Portugal",
        "Ghana",
        "USA",
        "Belgium",
        "Algeria",
        "Russia",
        "Korea Republic",
    ];

    println!("{:?}", countries);
    println!("count: {:?}", countries.len());

    my_answer_1(&countries);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct WordItem {
    word: String,
    first: char,
    last: char,
}

impl WordItem {
    fn new(word: &str) -> WordItem {
        let word = word.to_string();
        let _upper = word.to_uppercase();
        let mut chars = _upper.chars();
        WordItem {
            word: word,
            first: chars.nth(0).unwrap(),
            last: chars.last().unwrap(),
        }
    }

    fn is_match(&self, word: &WordItem) -> bool {
        self.last == word.first
    }
}


fn my_answer_1(countries: &[&'static str]) {
    let items = countries.iter().map(|x| WordItem::new(x) ).collect::<Vec<WordItem>>();
    let results: Vec<_> =
        items
        .iter()
        .flat_map(|item| {
            // println!("First: {:?}", item);
            let first = item;
            my_answer_1_search(vec![first], first, items.iter().filter(|x| *x != first).collect())
        })
        .fold(vec![], |mut acc, x| {
            acc.push(x);
            acc
        });
        // .collect();

    // results.iter().for_each(|x| println!("{:?}", x) );

    let max = results
        .iter()
        .max_by_key(|x| x.len() );

    println!("Results: {:?}", max.unwrap().iter().map(|x| x.word.as_str()).collect::<Vec<&str>>());
    println!("Count: {:?}", max.unwrap().len());
}

fn my_answer_1_search<'a> (used: Vec<&'a WordItem>, prev: &WordItem, extra: Vec<&'a WordItem>) -> Vec<Vec<&'a WordItem>> {
    // println!("----------------------------------------------------");
    // println!("used: {:?}", used);
    // println!("prev: {:?}", prev);
    // println!("extra: {:?}", extra);
    let a = extra.iter().filter(|x| prev.is_match(x)).flat_map(|item| {
        let mut _used = used.clone();
        _used.push(item);
        my_answer_1_search(
            _used,
            item,
            extra.iter().filter(|x| *x != item).cloned().collect::<Vec<&WordItem>>()
        )
    }).collect::<Vec<_>>();

    if a.len() > 0 {
        a
    } else {
        vec![used]
    }
}
