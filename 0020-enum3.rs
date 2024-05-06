#[derive(Debug)]

enum MobilePhones {
    Nokia,
    Motorola,
}

//enum & match

impl  MobilePhones {

    fn slogan(&self) -> String {

        match  self {

            MobilePhones::Nokia => String::from("Connecting people!"),
            MobilePhones::Motorola => String::from("Hello Moto!"),
            
        }
    }
}

fn main() {

    let phone = MobilePhones::Motorola;

    println!("{}", phone.slogan()); // Hello Moto!

}

