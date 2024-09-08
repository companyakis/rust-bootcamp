fn main() {

    let years: Vec<u16> = vec![1990, 1993, 2018, 2020, 2024, 2025];

    let search_year_one: Option<&u16> = years.get(4);

    let search_year_two: Option<&u16> = years.get(17);

    println!("Search year one: {:?}", search_year_one); // Search year one: Some(2024)

    println!("Search year two: {:?}", search_year_two); // Search year two: None

    // let's create a variable to hold the year info

    let mut expected_year: u16 = 0;

    match search_year_one {
        Some(year) => {
            println!("Year info: {year}"); // Year info: 2024
            expected_year = *year; // dereferencing
        }
        None => println!("No year info!")
    }

    println!("Expected year info: {expected_year}"); // Expected year info: 2024

}
