fn main() {

    let man = Man {name: String::from("Bilge")};

    man.speak(); // Bilge says: Hi. How are you?

    let cat = Cat {name: "Mırtık".to_string()};

    cat.speak(); // Mırtık says: Miyavvv:)
}

trait Speak {
    fn speak(&self);
}

struct Man {
    name: String
}

struct Cat {
    name: String
}

impl Speak for Man {
    fn speak(&self) {
        println!("{} says: Hi. How are you?", self.name);
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("{} says: Miyavvv:)", self.name);
    }
}
