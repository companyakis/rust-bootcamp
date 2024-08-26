fn main() {

    //let's use expect method => success or panic with our message

    let connection_num1: u8 = 1;

    let connection1 = database_connection(connection_num1).expect("Connection failure..."); // Connected

    print!("{connection1}"); // Connected


    let connection_num2: u8 = 3;

    let connection2 = database_connection(connection_num2).expect("Connection failure..."); 

    print!("{connection2}"); // panic and "Connection failure...: "Not connected!"

}


fn database_connection(val: u8) -> Result<String, String> {

    if val == 1 {

        Result::Ok("Connected".to_string())
    }
    else {
        Result::Err("Not connected!".to_string())
    }
}
