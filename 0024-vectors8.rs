fn main() {

    let mut nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];

    // let's use f(x) = 2 * x + 1 function

    for x in &mut nums {

        *x = 2 * *x + 1; // dereferencing
    }

    println!("Nums: {:?}", nums); // Nums: [3, 5, 7, 9, 11, 13, 15]

}




