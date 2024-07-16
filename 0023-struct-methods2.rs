fn main() {
    
    let rectangular_1: Rectangular = Rectangular { height: 12.0, length: 15.0};

    println!("Multiplied area: {}", rectangular_1.multiply_area(8.21)); // Multiplied area: 1477.8

}

struct Rectangular {
    height: f32,
    length: f32,
}

impl Rectangular {

    // let's add a new method
    fn multiply_area(&self, a: f32) -> f32 {
        self.height * self.length * a
    } 

    fn calculate_area(&self) -> f32 {
        self.length * self.height
    }

    fn calculate_perimeter(&self) -> f32 {
        2.0 * (self.height + self.length)
    }
}
