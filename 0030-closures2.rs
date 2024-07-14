fn main() {

    let multiply_numbers = |a: u64, b: u64, c: u64| -> u64 {
      
        a  * b * c
    };

    let x: u64 = 22;

    let y: u64= 11111;

    let z: u64= 2221;

    let result = multiply_numbers(x, y, z);

    println!("{x} * {y} * {z} = {result}"); // 22 * 11111 * 2221 = 542905682
    
}


