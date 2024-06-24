fn main() {

    // string concatenation

    let name = "Mustafa".to_string();

    let surname = "Büyükdereli".to_string();

    let year = "2024".to_string();

    let info = name + " " + &surname + " " + &year + ".";

    println!("{info}"); // Mustafa Büyükdereli 2024.

}

