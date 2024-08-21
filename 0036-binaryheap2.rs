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

    
    println!("Sample binary heap: {:?}", bh); // Sample binary heap: [1000, 22, 4, 3, 16, 0]

    // pop -> where is the max value:(

    bh.pop();

    println!("Sample binary heap: {:?}", bh); // Sample binary heap: [22, 16, 4, 3, 0] 

    bh.pop();

    println!("Sample binary heap: {:?}", bh); // Sample binary heap: [16, 3, 4, 0] 


}

