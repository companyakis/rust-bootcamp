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

    // find intersection items method 1

    for i in hs1.intersection(&hs2) {

        println!("Intersection: {}", i);
    }

    /*
    Intersection: 0
    Intersection: 12  
    */

    // find intersection items method 2

    let intersetion_between_hs = &hs1 & &hs2; // & -> AND

    println!("Intersection: {:?}", intersetion_between_hs); // Intersection: {0, 12}
 
}

