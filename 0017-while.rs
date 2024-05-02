fn main() {

    let mut counter: u8 = 0;

    while counter < 100 {

        println!("Counter: {counter}");

        counter = 2 * counter + 1;
    }

}

/*
Counter: 0
Counter: 1 
Counter: 3 
Counter: 7 
Counter: 15
Counter: 31
Counter: 63

*/

