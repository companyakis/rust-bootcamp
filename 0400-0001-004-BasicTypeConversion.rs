// Rust makes numeric type conversions very easy with the as keyword

fn main() {

    let x = 110u8;
    
    let y = 200u8;
    
    //let sum: u16 = x + y; // Error => mismatched types
    
    let sum: u16 = x as u16 + y as u16;
    
    println!("Sum = {sum}");

    let a = true;
    
    println!("{}", a as u8);
    
    let b = false;
    
    println!("{}", b as u8);
}
