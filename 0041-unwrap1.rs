fn main() {

    let years: Vec<u16> = vec![1990, 1993, 2018, 2020, 2024, 2025];

    let search_year_one = years.get(4).unwrap(); // 2024

    println!("{search_year_one}");

    let search_year_one_2 = years.get(4).copied().unwrap();

    println!("{search_year_one_2}"); // 2024

    // out of index!

    // let search_year_two= years.get(17).unwrap();

    // println!("{search_year_two}"); // Error!

    let search_year_two= years.get(17).copied().unwrap_or(1000);

    println!("{search_year_two}"); // 1000

    let search_year_three = years.get(25).copied().unwrap_or_default();

    println!("{search_year_three}"); // 0
    
}
