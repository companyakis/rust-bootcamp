fn main() {

    let her_birth_year = 1990; // immutable

    //her_birth_year = 1991; //cannot mutate immutable variable `her_birth_year`rust-analyzerE0384

    let mut his_birth_year = 1990; // mutable

    his_birth_year = 2000;

    println!("Her birth year is {her_birth_year}."); // Her birth year is 1990.

    println!("His birth year is {his_birth_year}."); // His birth year is 2000.
    
}

