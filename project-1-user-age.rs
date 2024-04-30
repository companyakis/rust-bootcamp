use std::io;
fn main() {

    println!("Enter your birth year: ");

    let mut user_birth = String::new();

    io::stdin()
        .read_line(&mut user_birth)
        .expect("Please, try again...");

    let int_user_birth: u16 = user_birth.trim().parse().unwrap();

    println!("Current year info: ");

    let mut current_year = String::new();

    io::stdin()
        .read_line(&mut current_year)
        .expect("Please, type this year info (2024, 2023 etc...");

    let int_current_year: u16 = current_year.trim().parse().unwrap();

    let user_age = int_current_year - int_user_birth;

    println!("You are {user_age} years old.");
  
}

/*
Enter your birth year:
1921
Current year info: 
2028
You are 107 years old.

*/

