#[derive(Debug)]
struct Employee<T, V> {
    general: T,
    id: V,
    info: T
}



fn main() {

    let e1 = Employee {general: "HR department".to_string(), id: 105, info: "Not married".to_string()};

    println!("Employee 1 info: {:?}", e1); // Employee 1 info: Employee { general: "HR department", id: 105, info: "Not married" }

    let e2 = Employee { general: 2018, id: "101".to_string(), info: 1959};

    println!("Employee 2 info: {:?}", e2); // Employee 2 info: Employee { general: 2018, id: "101", info: 1959 }

}
