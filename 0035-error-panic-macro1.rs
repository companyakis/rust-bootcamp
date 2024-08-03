fn main() {

    let v = vec![1..10];

    // When we try to run this, Rust stops execution and spits out a panic message 
  
    println!("{:?}", v[101]); // thread 'main' panicked at src\main.rs:5:23:

}

