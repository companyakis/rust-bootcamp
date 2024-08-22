fn main() {
    let employee1 = Employee {
        name: "Mustafa".to_string(),
        department: "FinTech".to_string(),
        wage: 100_000,
        id: 01,
    };

    println!("Employee Mustafa Info: {:#?}", employee1);

    let employee2 = Employee {
        name: "Aysel".to_string(),
        ..Default::default()
    };

    println!("Employee Aysel Info: {:#?}", employee2);
}

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
    wage: u32,
    id: u8,
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            name: "Employee Name".to_string(),
            department: "No info".to_string(),
            wage: 0,
            id: 0,
        }
    }
}

/*
Employee Mustafa Info: Employee {
    name: "Mustafa",
    department: "FinTech",
    wage: 100000,
    id: 1,
}
Employee Aysel Info: Employee {
    name: "Aysel",
    department: "No info",
    wage: 0,
    id: 0,
}

*/
