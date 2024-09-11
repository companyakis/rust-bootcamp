fn main() {

    let mut all_employee: Vec<Vec<EmployeeInfo>> = Vec::new();

    let employee_bumin: Vec<EmployeeInfo> = vec![
        EmployeeInfo::ID(101),
        EmployeeInfo::Department("Sales".to_string()),
        EmployeeInfo::Name("Bumin Satıcı".to_string()),
        EmployeeInfo::SalaryUSD(4500),
    ];

    let employee_aybuke: Vec<EmployeeInfo> = vec![
        EmployeeInfo::ID(187),
        EmployeeInfo::Department("Accounting".to_string()),
        EmployeeInfo::Name("Aybüke Hesapçı".to_string()),
        EmployeeInfo::SalaryUSD(4300),
    ];


    all_employee.push(employee_bumin);
    all_employee.push(employee_aybuke);

    println!("All employee info: {:?}", all_employee[0][0]); // All employee info: ID(101)

}

#[derive(Debug)]
enum EmployeeInfo {
    ID(u16),
    Department(String),
    Name(String),
    SalaryUSD(u16)
}

