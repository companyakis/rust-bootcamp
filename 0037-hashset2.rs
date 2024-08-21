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

    // item

    for i in hs.iter() {

        println!("İtem: {}", i);
    }

    /*
    İtem: -5      
    İtem: 2500
    İtem: 12
    İtem: 0  
    */

    
}

