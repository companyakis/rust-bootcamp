fn main() {

    let fintech_employee_mustafa = create_employee(
                                        String::from("Mustafa Gökkağan"), 
                                        String::from("FinTech"), 
                                        100000.0, 
                                        88, 
                                        false,
                                        2
                                    );

    println!("Name: {}", fintech_employee_mustafa.name);
    println!("Department: {}", fintech_employee_mustafa.department);    
    println!("Salary: {}", fintech_employee_mustafa.salary);    
    println!("Age: {}", fintech_employee_mustafa.age);    
    println!("Marriage: {}", fintech_employee_mustafa.marriage);    
    println!("Number of children: {}", fintech_employee_mustafa.number_of_children);                                  

}

struct Employee {
    name: String,
    department: String,
    salary: f32,
    age: u8,
    marriage: bool,
    number_of_children: u8,
}

fn create_employee(name: String, department: String, salary: f32, age: u8, marriage: bool, number_of_children: u8) -> Employee {

    Employee {name, department, salary, age, marriage, number_of_children}

}

/*
Name: Mustafa Gökkağan
Department: FinTech
Salary: 100000     
Age: 88
Marriage: false
Number of children: 2

*/
