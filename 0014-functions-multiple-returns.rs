fn main() {

    println!("Sum: {}", sum_substract(20132, 1814).0);

    println!("Subtract: {}", sum_substract(2024, 1990).1);

}

fn sum_substract(num1: i128, num2: i128) -> (i128, i128) {
   
    return (num1 + num2 , num1 - num2);
}

/*
Sum: 21946
Subtract: 34
*/

