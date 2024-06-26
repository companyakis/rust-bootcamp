fn main() {

    let x = 33;

    let y = 46;

    // let's use logical operators

    if x > y && x % 3 == 0 {
        println!("Perfect selection")
    }

    else if x == y || y % 5 == 1 {
        println!("Thank you")
    }

    else if !(x != y) {
        println!("Quit")
    }

}

