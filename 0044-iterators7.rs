fn main() {

    let nums: [i16; 5] = [-100, 0, 12, 500, 650];

    let iterator = nums.iter();

    // y = 2 * x - 1

    let func_iterator = iterator.map(|i| 2 * i - 1);

    // create a vector!

    let new_numbers: Vec<i16> = func_iterator.collect();

    println!("New numbers: {:?}", new_numbers); // New numbers: [-201, -1, 23, 999, 1299]


}


