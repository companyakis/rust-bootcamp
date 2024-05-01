fn main() {

    //mutability
    
    let mut years: [u16;5] = [2020, 2021, 2022, 2023, 2024];

    years[0] = 1990;

    println!("{years:?}"); //[1990, 2021, 2022, 2023, 2024]

}
