#[derive(Debug)]

enum MobilePhones {
    Nokia,
    Motorola,
    LG,
    Toshiba,
    Ericsson
}


//enum methods

impl  MobilePhones {

    fn nokia_says(&self) -> String {

        String::from("Connecting people.")
    }

    fn motorola_says(&self) -> String {

        String::from("Hello Moto!")
    }

}

fn main() {

    let moto = MobilePhones::Motorola;

    println!("{}", moto.motorola_says()); // Hello Moto!

}

