fn main() {

    let result:  i32 = sum(Some(145), Some(57), Some(-45));

    println!("Sum: {}", result); // Sum: 157

}

fn sum(x: Option<i32>, y: Option<i32>, z: Option<i32>) -> i32 {

    x.unwrap_or(0) + y.unwrap_or(0) + z.unwrap_or(0)

}


