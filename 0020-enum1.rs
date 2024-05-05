#[derive(Debug)]

enum MobilePhones {
    Nokia,
    Motorola,
    LG,
    Toshiba,
    Ericsson
}

fn main() {

    let phone = MobilePhones::Nokia;

    println!("In 2020, I bought my first mobile phone. It was {phone:?} 51-10...");

}

/*
In 2020, I bought my first mobile phone. It was Nokia 51-10...
*/

