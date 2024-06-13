use std::collections::HashMap;

fn main() {

    let mut salaries: HashMap<String, u32> = HashMap::new();

    salaries.insert(String::from("Mustafa"), 80000);

    salaries.insert(String::from("Aygun"), 50000);

    salaries.insert(String::from("Kagan"), 45000);

    // remove elements

    salaries.remove(&String::from("Kagan"));

    println!("Final HashMap: {:?}", salaries); // Final HashMap: {"Mustafa": 80000, "Aygun": 50000}
}

