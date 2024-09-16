fn main() {

    let pow_closure = |x: i16| x.pow(5);

    let result1 = pow_closure(3);

    println!("3^5: {:?}", result1); // 3^5: 243 

    let result2 = pow_closure(4);

    println!("4^5: {:?}", result2); // 4^5: 1024

}

