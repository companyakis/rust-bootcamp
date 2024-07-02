use std::collections::HashMap;

fn main() {

    let name1 = String::from("Mustafa");

    let salary1: u16 = 12500;

    let name2 = String::from("Teoman");

    let salary2: u16 = 9400;

    let mut salaries: HashMap<String, u16> = HashMap::new();

    salaries.insert(name1, salary1);

    println!("Salaries: {:?}", salaries); // Salaries: {"Mustafa": 12500}

    //println!("Name 1: {name1}"); // value borrowed here after move

    println!("Salary1: {salary1}"); // Salary1: 12500

    salaries.insert(name2.clone(), salary2);

    println!("Salaries: {:?}", salaries); // Salaries: {"Mustafa": 12500, "Teoman": 9400}

    println!("Name 2: {name2}"); // Name 2: Teoman

    println!("Salary2: {salary2}"); // Salary2: 9400

}
