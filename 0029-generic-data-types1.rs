fn main() {

    let numbers = [3, -12, 21, 321, -249];

    println!("Max number: {:?}", compare_nums(&numbers)); // Max number: 321

    let chars = ['x', 'a', 'g', 'y', 'j', 'z', 'b'];

    println!("Max char: {:?}", compare_chars(&chars)); // Max char: 'z'
}

// As you see, we have tho functions, but why not one?..

fn compare_nums(list: &[i16]) -> &i16 {
    let mut max_num = &list[0];

    for number in list {
        if number > max_num {
            max_num = number;
        }
    }
    max_num
}

fn compare_chars(list: &[char]) -> &char {
    let mut max_char = &list[0];

    for c in list {
        if c > max_char {
            max_char = c;
        }
    }
    max_char
}
