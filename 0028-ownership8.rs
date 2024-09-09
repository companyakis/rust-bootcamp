fn main() {

    // we can think => don't consider pointer but just consider data!

    let mut year: u16 = 2024;

    println!("This year: {year}");

    let current_year: &mut u16 = &mut year;

    println!("This year 2: {current_year}");

    //current_year = 2025; // Error => consider dereferencing here to assign to the mutably borrowed value

    //println!("This year 2: {current_year}");

    *current_year = 2025; // !!! => dereferencing

    println!("This year 2: {current_year}");

    println!("This year: {year}");

}

/*

This year: 2024  
This year 2: 2024
This year 2: 2025
This year: 2025  

*/



