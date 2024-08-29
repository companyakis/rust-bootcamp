fn main() {

    let proverb: &str = "A rolling stone gathers no moss";

    let search_item: char = 'o';

    let mut counter_char_o = 0;

    for c in proverb.chars() {

        if c == search_item {
            counter_char_o += 1;
        }
    }

    if counter_char_o > 0 {

        println!("We found {counter_char_o} times {search_item} in  proverb '{proverb}'."); // We found 4 times o in  proverb 'A rolling stone gathers no moss'.
    }

    else {

        println!("Zero...");
    }
}

