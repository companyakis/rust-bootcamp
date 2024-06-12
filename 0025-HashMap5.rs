use std::collections::HashMap;

fn main() {

    let mut salaries: HashMap<String, u32> = HashMap::new();

    salaries.insert(String::from("Mustafa"), 80000);

    salaries.insert(String::from("Aygun"), 50000);

    salaries.insert(String::from("Kagan"), 45000);

    // Iterating  

    for (k, v) in &salaries { //we should borrow

        println!("Name: {k} and salary: {v}");
    }
    
}

/*
Name: Mustafa and salary: 80000
Name: Aygun and salary: 50000
Name: Kagan and salary: 45000
*/
