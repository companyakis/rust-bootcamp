fn main() {

    let year: u16 = 2024; // stored on the stack 

    let name: String = "Mustafa".to_string(); // stored on the heap 

    let this_year = year;

    let my_name = name; // ownership of "name" is moved to "my_name"

    println!("{year}"); // 2024

    //println!("{name}"); // borrow of moved value: `name`rustc

    println!("{this_year}"); // 2024

    println!("{my_name}"); // Mustafa

}

