fn main() {

    // from stack memory to heap memory

    let year: Box<u16> = Box::new(2024);

    println!("Year (heap memory) : {year}");

    let year_2: u16 = 2024;

    println!("Year (stack memory) : {year_2}")

    // Year (heap memory) : 2024
    // Year (stack memory) : 2024
    
}


