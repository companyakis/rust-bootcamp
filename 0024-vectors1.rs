fn main() {

    let mut years = vec![2020, 2021, 2022, 2023, 2024];

    println!("Years: {:?}", years); // Years: [2020, 2021, 2022, 2023, 2024]

    years.push(2026);

    println!("Years: {:?}", years); // Years: [2020, 2021, 2022, 2023, 2024, 2026]

    years[0] = 2010;

    println!("Years: {:?}", years); // Years: [2010, 2021, 2022, 2023, 2024, 2026]
    
    
    for year in &years {
        println!("Year: {year}");
    }

}

    /*
Year: 2010
Year: 2021
Year: 2022
Year: 2023
Year: 2024
Year: 2026
    
    */

