fn main() {

    //fixed length and same data types
    
    let years: [u16;5] = [2020, 2021, 2022, 2023, 2024];

    let ids = [1, 2, 3, 4];

    println!("{years:?}");
  
    println!("{}", years[0]);
    println!("{}", years[1]);
    println!("{}", years[2]);
    println!("{}", years[3]);
    println!("{}", years[4]);

    let id = ids[2];

    println!("id: {id}",);
}

/*
[2020, 2021, 2022, 2023, 2024]
2020 
2021 
2022 
2023 
2024 
id: 3
*/

