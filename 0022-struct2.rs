fn main() {

    let employee_mustafa: Employee = Employee { name: "Mustafa".to_string(), title: "Founder".to_string(), department_id: 101};

    // look!
    // assume they are from the same department

    let employee_kutluk: Employee = Employee  { name: "Kutluk".to_string(), title: "CFO".to_string(), ..employee_mustafa};

    println!("Name: {}", employee_kutluk.name);

    println!("Title: {}", employee_kutluk.title);

    println!("Name: {}", employee_kutluk.department_id);
    
}

struct Employee {
    
    name: String,
    title: String,
    department_id: u8,

}

/*
Name: Kutluk
Title: CFO
Name: 101
*/
