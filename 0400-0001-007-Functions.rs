fn main() {

println!("Sum: {}", sum(1, 2, -5));

println!("Squared Sum: {}", squared_sum(1, 2, -5))

// Sum: -2
// Squared Sum: 30

}

fn sum(x: i32, y: i32, z: i32) -> i32 {
    
    x + y + z
}

fn squared_sum(x: i32, y: i32, z: i32) -> i32 {
    
    x * x + y * y + z * z
}
