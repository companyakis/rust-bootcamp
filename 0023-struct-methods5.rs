fn main() {

    let mut rec1 = Rectangular {
        width: 12,
        height: 11
    };

    println!("Area 1: {:?}", rec1.calculate_area()); // Area 1: 132

    // mut?

    rec1.change_width(15);

    println!("Area 2: {:?}", rec1.calculate_area()); // Area 2: 165


}

struct Rectangular {
    width: u32,
    height: u32,
}

impl Rectangular {

    fn calculate_area(&self) -> u32 {

        self.height * self.width
    }

    fn change_width(&mut self, new_width: u32) {
        
        self.width = new_width
    }
    
}
