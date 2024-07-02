use std::collections::HashMap;

fn main() {

    let salaries = HashMap::from([
        ("Mustafa", 9000),
        ("Aygun", 6500),
        ("Gokturk", 7000)
    ]);

    let salary_query1 = salaries.get("Kagan");

    match  salary_query1 {
        Some(info) => println!("Salary info: {}", info),
        None => println!("No user..."), // No user...
    }

    let salary_query2 = salaries.get("Mustafa");

    match salary_query2 {
        Some(info) => println!("Salary info: {info}"), // Salary info: 9000
        None => println!("No user..."),
        
    }

}
