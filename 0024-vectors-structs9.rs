struct Employee {
    name: String,
    id: u16,
    department: String
}

fn main() {

    let employees = vec![

            Employee{name: String::from("Mustafa"), id: 1, department: String::from("FinTech")},
            Employee{name: String::from("Aykız"), id: 1211, department: String::from("Sales")},
            Employee{name: String::from("Gökhan"), id: 127, department: String::from("Marketing")},
            Employee{name: String::from("Kağan"), id: 12, department: String::from("Finance")},
            Employee{name: String::from("Mustafa"), id: 101, department: String::from("Accounting")},
    ];

    for e in employees {

        println!("Name: {}, ID: {}, department: {}", e.name, e.id, e.department);
    }
}

/*

Name: Mustafa, ID: 1, department: FinTech
Name: Aykız, ID: 1211, department: Sales      
Name: Gökhan, ID: 127, department: Marketing  
Name: Kağan, ID: 12, department: Finance      
Name: Mustafa, ID: 101, department: Accounting

*/
