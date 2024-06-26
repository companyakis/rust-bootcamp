fn main() {

    let mut string_1 = String::from("Mustafa");

    let string_2 = &mut string_1;

    string_2.push_str(" Buyukdereli");

    let string_3 = &mut string_1;

    string_3.push_str(" was here.");

    println!("{string_1}"); // Mustafa Buyukdereli was here.

    //println!("{string_2}");

    //println!("{string_3}");
    
}

