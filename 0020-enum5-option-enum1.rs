fn main() {

    let year_info: Option<u16> = Some(2024);

    let is_completed: Option<bool> = Some(false);

    let founder_name: Option<String> = Some("Mustafa".to_string());

    let new_employee_name: Option<&str> = None;

    println!("{:?}", year_info);
    
    println!("{:?}", is_completed);

    println!("{:?}", founder_name);

    println!("{:?}", new_employee_name);
}

/*
Some(2024)     
Some(false)    
Some("Mustafa")
None
*/
