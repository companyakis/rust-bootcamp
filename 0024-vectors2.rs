fn main() {

    let mut cities: Vec<String> = Vec::new();

    cities.push(String::from("Izmir"));

    cities.push(String::from("Ankara"));

    cities.push(String::from("Istanbul"));

    let my_university_city = &cities[0];

    println!("Cities: {:?}", cities);

    println!("My university city is {}.", my_university_city);

}

/*
Cities: ["Izmir", "Ankara", "Istanbul"]
My university city is Izmir.
*/
