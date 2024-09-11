fn main() {

    // dereference

    let a: u8 = 101;

    let b: Box<u8> = Box::new(a);

    let result = a == *b;

    println!("Equal?: {:?}", result); // Equal?: true

    let sum = a + *b;

    println!("value of (a + b) => {sum}"); // value of (a + b) => 202

}


