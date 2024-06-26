fn main() {

    let mut year: u16 = 2024;

    let year_2 = &mut year;

    *year_2 = 2023;

    println!("{}", year_2); // 2023

    println!("{}", year); // 2023

    year = 2025;

   println!("{}", year); // 2025
}

