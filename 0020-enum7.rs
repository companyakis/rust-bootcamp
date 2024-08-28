fn main() {

    let people1 = People::Retired;
    
    let emploee_kagan = Employee {id: 141, name: "Kağan Kul".to_string()};
    
    let people2 = People::Working(emploee_kagan);

    println!("{:#?}", people1);
    
     println!("{:#?}", people2);

}

#[derive(Debug)]

struct Employee {
    id: u16,
    name: String
}
#[derive(Debug)]
enum People {
    Retired,
    Children,
    Working(Employee)
}

// Retired
// Working(
//     Employee {
//         id: 141,
//         name: "Kağan Kul",
//     },
// )
