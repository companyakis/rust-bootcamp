use std::io;

// employee info input

fn employee_info_input() -> String {

    let mut data: String::new();

    io::stdin().read_line(&mut data).expect("Please, try again!");

    data
}
