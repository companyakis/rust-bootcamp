fn main() {

    let c1: PlayerCoordinates<u8> = PlayerCoordinates {x: 5, y: 12, z: 4};

    c1.coordinates_info(); // Coordinates => x: 5 - y: 12 - z: 4

    let c2: PlayerCoordinates<f32> = PlayerCoordinates {x: 12.3, y: 8.45, z: 13.23};

    c2.coordinates_info(); // Coordinates => x: 12.3 - y: 8.45 - z: 13.23

}

struct PlayerCoordinates<T: std::fmt::Display> {
    x: T,
    y: T,
    z: T
}

impl <T: std::fmt::Display> PlayerCoordinates<T> {
    fn coordinates_info(&self) {
        println!("Coordinates => x: {} - y: {} - z: {}", &self.x, &self.y, &self.z);
    }
}
