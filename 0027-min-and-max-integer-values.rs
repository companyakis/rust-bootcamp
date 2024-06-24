fn main() {

    /*
    
    int an uint data types max and min values

    int => from -(2 ** (n - 1)) to (2 ** (n-1) - 1)

    uint => from (0) to (2 ** n - 1)
    
    */

    let min_value = - i32::pow(2, 7);

    println!("i8 data-type min value: {:?}", min_value);

    println!("i8 data-type min value: {:?}", i8::MIN);

    let max_value = i32::pow(2, 7) - 1;

    println!("i8 data-type max value: {:?}", max_value);

    println!("i8 data-type max value: {:?}", i8::MAX);

}

/*
i8 data-type min value: -128
i8 data-type min value: -128
i8 data-type max value: 127
i8 data-type max value: 127
*/
