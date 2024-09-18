fn main() {

    let years: [u16; 6] = [2018, 2019, 2020, 2021, 2022, 2023];

    let mut year_iterator = years.iter();

    // next method

    println!("Year: {:?}", year_iterator.next().unwrap_or(&0));
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());
    println!("Year: {:?}", year_iterator.next());

// Year: 2018
// Year: Some(2019)
// Year: Some(2020)
// Year: Some(2021)
// Year: Some(2022)
// Year: Some(2023)
// Year: None      
// Year: None    

}


