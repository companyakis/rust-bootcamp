fn main() {
    
    let rectangular_1: Rectangular = Rectangular { height: 12.0, length: 15.0};

    println!("Area of the rectangular 1: {}", rectangular_1.calculate_area()); // Area of the rectangular 1: 180
    
    println!("Perimeter of the rectangular 1: {}", rectangular_1.calculate_perimeter()); // Perimeter of the rectangular 1: 54
}

struct Rectangular {
    height: f32,
    length: f32,
}

impl Rectangular {

    fn calculate_area(&self) -> f32 {
        self.length * self.height
    }

    fn calculate_perimeter(&self) -> f32 {
        2.0 * (self.height + self.length)
    }
}

