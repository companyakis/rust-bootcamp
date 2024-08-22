fn main() { 

    let employee1 = Employee {
        name: "Hakan Kağan".to_string(),
        department: "Finance".to_string(),
        wage: 45_000,
        id: 12,
    };

    // assume some data is lost!
    // "default" values can be used

    let employee2 = Employee{
        name: "Bilge Kutlular".to_string(),
        ..Default::default()
    };

    println!("Employee 1: {:#?}", employee1);

    println!("Employee 2: {:#?}", employee2);

}

// default!

#[derive(Debug, Default)]
struct Employee {
    name: String,
    department: String,
    wage: u16,
    id: u8
}

/*

Employee 1: Employee {
    name: "Hakan Kağan",
    department: "Finance",
    wage: 45000,
    id: 12,
}
Employee 2: Employee {
    name: "Bilge Kutlular",
    department: "",
    wage: 0,
    id: 0,
}

*/
