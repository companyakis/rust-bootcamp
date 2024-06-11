fn main() {

    let department: Departmments = Departmments::Operations;

    let employee_number: u8 = match department {

        Departmments::FinTech => 12,
        Departmments::Finance => 5,
        Departmments::Operations => 3,
        Departmments::Sales => 4,
        Departmments::Marketing => 4,
        Departmments::Risk => 3,
        Departmments::HR => 2
    };

    println!("Number of employee in department {:?}: {:?}.", department, employee_number); // Number of employee in department Operations: 3.

}

#[derive(Debug)]
enum Departmments {
    FinTech,
    Finance,
    Operations,
    Sales,
    Marketing,
    Risk,
    HR
}

/*
Some(2024)     
Some(false)    
Some("Mustafa")
None
*/
