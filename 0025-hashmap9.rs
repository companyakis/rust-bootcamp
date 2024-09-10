use std::collections::HashMap;

fn main() {

    let mut final_result: HashMap<String, u8> = HashMap::new();

    final_result.insert("Aygün".to_string(), 92);

    final_result.insert("Kağan".to_string(), 86);

    final_result.insert("Mustafa".to_string(), 97);

    println!("Student final results: {:?}", final_result); // Student final results: {"Kağan": 86, "Mustafa": 97, "Aygün": 92}

    // let's try entry() and or_insert()

    final_result.entry("Kuzgun".to_string()).or_insert(18);

    println!("Student final results: {:?}", final_result); // Student final results: {"Aygün": 92, "Kuzgun": 18, "Kağan": 86, "Mustafa": 97}


}


/*

*/



