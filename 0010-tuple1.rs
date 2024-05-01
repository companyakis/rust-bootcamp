fn main() {

    // length is fixed
    // a tuple can hold diff. dtypes

    //name, surname, id, is_young

    let founder_info = ("Mustafa", "Büyükdereli", 1, true); //immutable

    let (name, surname, id, is_young) = founder_info;

    println!("Name: {name}");
    println!("Surname: {surname}");
    println!("ID: {id}");
    println!("Is he young: {is_young}");

    println!("Founder info tuple: {founder_info:?}");
}

/*
Name: Mustafa
Surname: Büyükdereli
ID: 1
Is he young: true
Founder info tuple: ("Mustafa", "Büyükdereli", 1, true)

*/

