fn main() {
    
    //let's call the struct method

    Employee::introduction(); // Please, don't share employee information anywhere!

}

struct Employee {
    name: String,
    id: u16,
    department: String
}

impl Employee {

    fn introduction() {

        println!("Please, don't share employee information anywhere!")
    }
    
}
