fn main() {

    let years: Vec<u16> = vec![1990, 1993, 2018, 2020, 2024, 2025];

    // this code part works, but we should consider memory management
    // for year in years {
    //     println!("Year => {year}");
    // }

    // memory!

    for year in &years {
        println!("Year => {year}");
    }

    let mut nums: Vec<u16> = vec![100, 140, 170, 200, 250, 300];

    let mut index = 0;

    for n in &mut nums {

        *n -= 5;

        index += 1;

        println!("New number_{} = {}", index, n);
 
    }

}

/*

Year => 1990      
Year => 1993      
Year => 2018      
Year => 2020      
Year => 2024      
Year => 2025      
New number_1 = 95 
New number_2 = 135
New number_3 = 165
New number_4 = 195
New number_5 = 245
New number_6 = 295

*/



