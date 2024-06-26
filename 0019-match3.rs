fn main() {

    let exam_result: u8 = 97;

    match exam_result {

        0..= 80 => println!("You should study hard!"),
        81..= 100 => println!("Good reult!"),
        _ => println!("What do you mean?"),
    }
}

