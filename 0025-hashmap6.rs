use std::collections::HashMap;

fn main() {

    let mut salaries= HashMap::from([
        ("mustafa", 8500),
        ("aygun", 5000),
        ("bilge", 4500),
        ("kultigin", 4600),
    ]);

    println!("Salary of bilge: {:?}", salaries["bilge"]); // Salary of bilge: 4500

    salaries.insert("teoman", 7000);

    println!("Salaries: {:?}", salaries); // Salaries: {"bilge": 4500, "kultigin": 4600, "teoman": 7000, "mustafa": 8500, "aygun": 5000}

}




