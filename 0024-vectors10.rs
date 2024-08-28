fn main() {

    let mut years = vec![2020, 2021, 2022, 2023, 2024];
    
    let mut year_iter = years.iter();
    
    while let Some(y) = year_iter.next() {
        
        print!("year: {y} ");
    }
}

// year: 2020 year: 2021 year: 2022 year: 2023 year: 2024 

