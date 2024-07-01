fn main() {

    let mut nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];

    let second_num = &nums[1]; // borrowing

    // //Error! Firstly, use second_num
    // nums.push(7);
    // println!("The second number: {second_num}"); //Error

    println!("The second number: {second_num}"); // The second number: 2

    nums.push(8);

    println!("Numbers: {:?}", nums); // Numbers: [1, 2, 3, 4, 5, 6, 7, 8]

}




