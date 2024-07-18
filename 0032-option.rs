struct Employee {
    name: String,
    idx: u16,
    department: String,
    title: Option<String>,
}

fn main() {
    let e1 = Employee {
        name: "Mustafa".to_string(),
        idx: 1,
        department: "FinTech".to_string(),
        title: Some("Partner".to_string()),
    };

    let e2 = Employee {
        name: "Bengü".to_string(),
        idx: 121,
        department: "Sales".to_string(),
        title: None,
    };

    let e3 = Employee {
        name: "Hakan".to_string(),
        idx: 92,
        department: "HR".to_string(),
        title: None,
    };

    let all_employee = [e1, e2, e3];

    for e in all_employee {
        match e.title {
            Some(t) => println!("Title info: {}", t),
            None => println!("No title info!") 
        }
    }
}

/*

Title info: Partner
No title info!
No title info!

*/
