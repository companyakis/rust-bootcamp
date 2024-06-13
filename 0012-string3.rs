fn main() {

    // slicing

    let who_am_i = String::from("Mustafa Büyükdereli");

    let my_first_name = &who_am_i[0..7];

    println!("{my_first_name}"); // Mustafa
}

