fn main() {

    let player1_coord: PlayerCoordinates<f32> = PlayerCoordinates { x: 3.9, y: 12.8, z: 4.0};

    let player2_coord: PlayerCoordinates<i16> = PlayerCoordinates {x: -5, y: 12, z: 4};

}

struct PlayerCoordinates<T> {
    x: T,
    y: T,
    z: T
}
