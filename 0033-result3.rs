fn main() {

    let connection_num1: u8 = 1;

    let connection1: Result<String, String> = database_connection(connection_num1);

    match connection1 {
        Ok(t) => println!("{t}"), // Connected
        Err(e) => println!("{e}"),
    }

    let connection_num2: u8 = 5;

    let connection2: Result<String, String> = database_connection(connection_num2);

    match connection2 {
        Ok(t) => println!("{t}"),
        Err(e) => println!("{e}"), // Not connected!
    }

}

// // Result Enum Error Management
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

fn database_connection(val: u8) -> Result<String, String> {

    if val == 1 {

        Result::Ok("Connected".to_string())
    }
    else {
        Result::Err("Not connected!".to_string())
    }
}
