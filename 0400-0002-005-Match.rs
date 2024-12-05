fn main() {

    let month: u8 = 9;

    match month {
        
        1 => {println!("January... First month...");}
        
        2 | 3 => {println!("Cold months...");}
        
        4 | 5 => {println!("Spring...");}
        
        6..=8 => {println!("Summer months...");}
        
        9..=12 => {println!("Time elapses fast!");}
        
        _ => {println!("Use a number b/w 1 and 12!");}
    }

}
