fn main() {

    //Dangling References

    let x;
  
    {
        let z: String = "Hi there".to_string();
        x = &z;
        println!("x: {}", x); // x: Hi there
    }

    //println!("x: {}", x); // Error

}

