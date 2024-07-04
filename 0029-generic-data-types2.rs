fn main() {

    let numbers = [3, -12, 21, 321, -249];

    println!("Max number: {:?}", compare_nums_or_chars(&numbers)); // Max number: 321

    let chars = ['x', 'a', 'g', 'y', 'j', 'z', 'b'];

    println!("Max char: {:?}", compare_nums_or_chars(&chars)); // Max char: 'z'

}

// Use generic data types to create one function

fn compare_nums_or_chars<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    let mut max_element = &list[0];

    for element in list {
        if element > max_element {
            max_element = element;
        }
    }
    max_element
}

