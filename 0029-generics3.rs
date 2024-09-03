fn main() {

    let numbers1: Vec<i8> = vec![-25, 12, -35, 87];

    println!("Min value 1: {:?}", find_min_value(&numbers1)); // Min value 1: -35

    let numbers2: Vec<u16> = vec![100, 99, 2000, 87, 96];

    println!("Min value 2: {:?}", find_min_value(&numbers2)); // Min value 2: 87

    let numbers3: Vec<f32> = vec![-10.8, 21.9, 33.7, -42.8, 999.21, -877.35];

    println!("Min value 3: {:?}", find_min_value(&numbers3)); // Min value 3: -877.35

}

fn find_min_value<T: std::cmp::PartialOrd>(values: &[T]) -> &T {

    let mut min_val = &values[0];

    for v in values {
        if v < min_val {
            min_val = v;
        }
    }
    min_val // return minumum value
}
