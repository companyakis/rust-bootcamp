use std::collections::BinaryHeap;

fn main() {

    let mut bh: BinaryHeap<u16> = BinaryHeap::new();

    // push

    bh.push(3);
    bh.push(22);
    bh.push(4);
    bh.push(16);
    bh.push(1000);
    bh.push(0);

    // max value is the first value!

    println!("Sample bibary heap: {:?}", bh); // Sample bibary heap: [1000, 22, 4, 3, 16, 0]

}

