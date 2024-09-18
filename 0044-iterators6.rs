fn main() {

    //mustafa buyukdereli was here

    let nums: [i16; 5] = [-100, 0, 12, 500, 650];

    let iterator = nums.iter();

    // y = 2 * x - 1

    let func_iterator = iterator.map(|i| 2 * i - 1);

    for element in func_iterator {
        println!("New element: {}", element);
    }

// New element: -201
// New element: -1  
// New element: 23  
// New element: 999 
// New element: 1299


}


