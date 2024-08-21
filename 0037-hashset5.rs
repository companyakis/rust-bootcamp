use std::collections::HashSet;

fn main() {

    let mut hs1: HashSet<i16> = HashSet::new();

    let mut hs2: HashSet<i16> = HashSet::new();

    hs1.insert(12);
    hs1.insert(-5);
    hs1.insert(0);
    hs1.insert(2500);
    hs1.insert(29);

    hs2.insert(0);
    hs2.insert(100);
    hs2.insert(-3000);
    hs2.insert(12);
    hs2.insert(324);
    hs2.insert(75);

    // union -> | OR

    let union_hs = &hs1 | &hs2;

    println!("Union: {:?}", union_hs); // Union: {75, -3000, 100, 324, 12, 2500, 29, 0, -5}

    // difference

    let diff_hs = &hs1 - &hs2;

    println!("Diff: {:?}", diff_hs); // Diff: {-5, 2500, 29}

}

