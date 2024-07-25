fn main() {

    let result1 = nothing_return();

    let result2 = nothing_return2();

    println!("Result 1: {:?}", result1);

    println!("Result 2: {:?}", result2);
}

fn nothing_return() -> () {
    return ();
}

fn nothing_return2() {

}

/*
Result 1: ()
Result 2: ()
*/
