fn main() {

    let counter_up = |x: i64| x + 1;

    let counter_down = |x: i64| x - 1;

    let number: i64 = 545_212;

    println!("Number ++: {:?}", counter_up(number)); // Number ++: 545213

    println!("Number --: {:?}", counter_down(number)); // Number --: 545211

}


