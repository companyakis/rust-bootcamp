#[derive(Debug)]
struct Employee<T> {
    name: String,
    id: u16,
    info: T
}

#[derive(Debug)]
struct City<T> {
    data1: T,
    data2: T
}

fn main() {

    let employee_hakan = Employee {name: "Hakan".to_string(), id: 101, info: true};

    println!("Hakan: {:?}", employee_hakan); // Hakan: Employee { name: "Hakan", id: 101, info: true }

    let employee_aybilge = Employee {name: "Aybilge".to_string(), id: 112, info: "Ankara".to_string()};

    println!("Aybilge: {:?}", employee_aybilge); // Aybilge: Employee { name: "Aybilge", id: 112, info: "Ankara" }

    let izmir = City {data1: "Ege".to_string(), data2: "İzmir Büyüşehir Belediyesi".to_string()};

    println!("İzmir: {:?}", izmir); // İzmir: City { data1: "Ege", data2: "İzmir Büyüşehir Belediyesi" }

    let ankara = City {data1: 2, data2: 5_500_000}; 

    println!("Ankara: {:?}", ankara); // Ankara: City { data1: 2, data2: 5500000 }
}
