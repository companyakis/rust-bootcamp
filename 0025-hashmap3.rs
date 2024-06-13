use std::collections::HashMap;

//key-value pairs, like Python dictionaries

fn main() {

    let mut salaries: HashMap<String, u32> = HashMap::new();

    salaries.insert(String::from("Mustafa"), 80000);

    salaries.insert(String::from("Aygun"), 50000);

    salaries.insert(String::from("Kagan"), 45000);

    // update elements

    salaries.insert(String::from("Mustafa"), 95000);

    println!("Updated salary of Mustafa: {:?}", salaries.get(&String::from("Mustafa"))); // Updated salary of Mustafa: Some(95000)
}

