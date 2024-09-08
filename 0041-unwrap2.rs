fn main() {

    let names: Vec<String> = vec!["Mustafa".to_string(), "Mete".to_string(), "Bilge".to_string(), "Kültigin".to_string()];

    let search_name = names.get(1).clone().unwrap();

    println!("{search_name}"); // Mete

    // out of index!

    // let search_name_2 = names.get(100).clone().unwrap();

    // println!("{search_name_2}"); // Error!

    let search_name_3 = names.get(100).clone().unwrap_or(&"Unknown name".to_string()).to_string();

    println!("{search_name_3}"); // Unknown name
}
