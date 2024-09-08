fn main() {

    let nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

    println!("1. element: {}", &nums[0]); // 1. element: 1

    println!("2. element: {}", &nums[1]); // 2. element: 2

    // But!

    //println!("100. element: {}", &nums[99]); // error: process didn't exit successfully

    let element_100: Option<&u8> = nums.get(99);

    println!("100. element: {:?}", element_100); // 100. element: None

}


