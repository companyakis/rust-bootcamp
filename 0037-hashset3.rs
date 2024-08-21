use std::collections::HashSet;

fn main() {

    let mut hs: HashSet<i16> = HashSet::new();

    hs.insert(12);
    hs.insert(-5);
    hs.insert(0);
    hs.insert(2500);
    hs.insert(12);
    hs.insert(12);
    hs.insert(12);

    println!("Sample hash set: {:?}", hs); // Sample hash set: {12, 0, 2500, -5}

    // remove

    hs.remove(&12);

    println!("Sample hash set: {:?}", hs); // Sample hash set: {0, 2500, -5}  
 
}

