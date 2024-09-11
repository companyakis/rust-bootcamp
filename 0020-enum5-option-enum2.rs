fn main() {

    // assume we have three inputs, but we aren't sure the values

    let a: u16 = 77;

    let b: u16 = 2021;

    //let c: Option<u16> = Some(17);

    let c: Option<u16> = None;

    let sum: u16;

    match c {

        Some(c) => println!("a + b + c = {}", a + b + c),
        None => println!("a + b = {}", a + b ),
        
    }

}


