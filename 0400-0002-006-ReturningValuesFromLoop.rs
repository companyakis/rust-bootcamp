fn main() {

    let mut year: u16 = 2020;
    
    let year_plus = loop {
        
        year += 1;
        
        if year == 2025 {
            
            break "2025 must be a good year!";
        }
    };
    
    println!("Next year message: {year_plus}")
    
}

// Next year message: 2025 must be a good year!

