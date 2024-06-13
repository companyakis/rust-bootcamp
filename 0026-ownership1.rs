fn main() {

    let my_name = String::from("Mustafa");

    println!("My name is {my_name}."); // My name is Mustafa.

    say_hello(my_name); // Hello, Mustafa!

    // Error!

    //println!("My name is {my_name}"); // rror[E0382]: borrow of moved value: `my_name` 

}

fn say_hello(name: String) {
    println!("Hello, {name}!");
}
