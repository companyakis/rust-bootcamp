fn main() {

    let mut v1 = vec![3, 4, 6, 12, 21];

    let slice_v1 = &v1[0..2];

    println!("Slice 1: {:?}", slice_v1); // Slice 1: [3, 4]

    println!("Vector 1: {:?}", v1); // Vector 1: [3, 4, 6, 12, 21]

}

