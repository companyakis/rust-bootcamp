use std::collections::HashMap;

fn main() {

    // hasmap keys, values and iter

    let mut salaries: HashMap<String, u32> = HashMap::new();

    salaries.insert(String::from("Mustafa"), 80000);

    salaries.insert(String::from("Aygun"), 50000);

    salaries.insert(String::from("Kagan"), 45000);

    // iter()  

    for (k, v) in salaries.iter() { 

        println!("Name: {k} and salary: {v}");
    }

    // keys()

    for k in salaries.keys() {
        println!("Key: {}", k);
    }
    
    // values()

    for k in salaries.values() {
        println!("Value: {}", k);
    }
    
}
