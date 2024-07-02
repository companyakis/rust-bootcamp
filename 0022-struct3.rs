fn main() {

    // Mutable Instance

    let mut employee_hakan = Employee {
        name: String::from("Hakan"),
        id: 101,
        department: String::from("Audit"),
    };

    employee_hakan.id = 102;

    println!("Name: {}", employee_hakan.name);
    println!("ID: {}", employee_hakan.id);
    println!("Department: {}", employee_hakan.department);
}

struct Employee {
    name: String,
    id: u16,
    department: String
}

/*
Name: Hakan
ID: 102
Department: Audit

*/
