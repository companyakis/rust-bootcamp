use std::collections::HashSet;

fn main() {

    let mut hs: HashSet<i16> = HashSet::new();

    println!("Length: {:?}", hs.len()); // Length: 0

    println!("Is empty: {:?}", hs.is_empty()); // Is empty: true

    hs.insert(12);
    hs.insert(-5);
    hs.insert(0);
    hs.insert(2500);
    hs.insert(12);
    hs.insert(12);
    hs.insert(12);

    // Only one "12"!

    println!("Sample hash set: {:?}", hs); // Sample hash set: {12, -5, 2500, 0}

}

