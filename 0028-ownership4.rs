fn main() {

    // Mutable borrowing

    let mut string_1 = String::from("Mustafa");

    let string_2 = &mut string_1;

    println!("{string_2}"); // Mustafa

    string_2.push_str(" Buyukdereli");

    //println!("{string_1}"); // cannot borrow `string_1` as immutable because it is also borrowed as mutable

    println!("{string_2}"); // Mustafa Buyukdereli
}

