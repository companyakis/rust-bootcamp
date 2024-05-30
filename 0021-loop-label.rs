fn main() {
    let mut a: u8 = 0;

    let mut b: u8 = 0;

    'outer_loop: loop {
      
        'inner_loop: loop {
          
            if a == 13 || b == 4 {
                
                break 'outer_loop;
            }

            a += 1;
            b += 5;
        }
    }

    println!("{a}");

    println!("{b}");
}
