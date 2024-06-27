fn main() {

    // function parameters copy and move example

    let year: u16 = 2024;

    let name = String::from("Mustafa");

    //print_parameters(year, name); // Name: Mustafa and year: 2024

    // Error!

    //println!("Name: {name}"); // borrow of moved value: `name`

    print_parameters_version_two(year, &name); // Name: Mustafa and year: 2024

    println!("Name: {name}"); // Name: Mustafa

}

fn print_parameters(year: u16, name: String) {
    println!("Name: {name} and year: {year}")
}

// Borrowing!
fn print_parameters_version_two(year: u16, name: &String) {
    println!("Name: {name} and year: {year}")
}
