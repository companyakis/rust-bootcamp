fn main() {

    let player1_coord: PlayerCoordinates<f32, u8, i16> = PlayerCoordinates { x: 3.9, y: 12, z: -5};

    println!("Player 1 x coord: {:?}", player1_coord.x);

    println!("Player 1 y coord: {:?}", player1_coord.y);

    println!("Player 1 z coord: {:?}", player1_coord.z);

}

struct PlayerCoordinates<T, U, V> {
    x: T,
    y: U,
    z: V
}

// Player 1 x coord: 3.9
// Player 1 y coord: 12
// Player 1 z coord: -5
