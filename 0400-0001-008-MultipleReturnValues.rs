fn main() {

    let result = calculator(5, 3);
    
    println!("a + b = {}", result.0);
    
    println!("a - b = {}", result.1);
    
    println!("a * b = {}", result.2);
    
    println!("a / b = {}", result.3)
    
// a + b = 8
// a - b = 2
// a * b = 15
// a / b = 1.6666666
    
 }   


fn calculator(a: i32, b: i32) -> (i32, i32, i32, f32) {
    
    return (a + b, a - b, a * b, a as f32 / b as f32)
}
