fn main() {

    let arr: [i8; 10] = [-5, -4, -3, -2, 1, 0, 1, 2, 3, 4];

    let slice_1_arr = &arr[2..5];

    println!("Slice 1: {:?}", slice_1_arr);

    let slice_2_arr = &arr[2..=5];

    println!("Slice 2: {:?}", slice_2_arr);

}

/*
Slice 1: [-3, -2, 1]
Slice 2: [-3, -2, 1, 0]
*/

