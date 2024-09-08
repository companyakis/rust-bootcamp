fn main() {

    let nums: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

    let search_item_1: Option<&u8> = nums.get(2);

    let search_item_2: Option<&u8> = nums.get(50);

    match search_item_1 {
        Some(item) => println!("Search item: {item}"),
        None => println!("Out of index... Please, use a correct index number!")
    }

    match search_item_2 {
        Some(item) => println!("Search item: {item}"),
        None => println!("Out of index... Please, use a correct index number!")
    }
}

/*
Search item: 3
Out of index... Please, use a correct index number!
*/


