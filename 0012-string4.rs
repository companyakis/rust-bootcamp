fn main() {

    // slicing 

    let who_am_i = String::from("Mustafa Büyükdereli");

    let surname = &who_am_i[7..];

    println!("{surname}"); // Büyükdereli
}
