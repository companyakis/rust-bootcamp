use std::collections::HashMap;

fn main() {

    let mut final_result: HashMap<String, u8> = HashMap::new();

    final_result.insert("Aygün".to_string(), 92);

    final_result.insert("Kağan".to_string(), 86);

    final_result.insert("Mustafa".to_string(), 97);

    // get()

    let search_person_1 = String::from("Tuna");

    let tuna_result = final_result.get(&search_person_1).copied().unwrap_or(0);

    println!("Tuna's final result: {tuna_result}"); // Tuna's final result: 0

    let search_person_2 = String::from("Aygün");

    let aygun_result = final_result.get(&search_person_2).copied().unwrap_or(0);

    println!("Aygün's final result: {aygun_result}"); // Aygün's final result: 92

}




