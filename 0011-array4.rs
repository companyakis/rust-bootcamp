fn main() {

    let mut years: [u16; 5] = [0; 5];

    println!("Array 1: {:?}", years);

    years[0] = 1990;
    years[1] = 2020;
    years[2] = 2024;
    years[3] = 1923;
    years[4] = 2025;

    println!("Array 2: {:?}", years);
}

/*
Array 1: [0, 0, 0, 0, 0]
Array 2: [1990, 2020, 2024, 1923, 2025]

*/
