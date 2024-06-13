use std::collections::HashMap;

//key-value pairs, like Python dictionaries

fn main() {

    let mut salaries: HashMap<String, u32> = HashMap::new();

    salaries.insert(String::from("Mustafa"), 80000);

    salaries.insert(String::from("Aygun"), 50000);

    salaries.insert(String::from("Kagan"), 45000);

    // access elements

    let mustafa_salary = salaries.get(&String::from("Mustafa"));

    println!("Mustafa's salary: {:?}", mustafa_salary); // Mustafa's salary: Some(80000)

}

