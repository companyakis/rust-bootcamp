fn main() {

    // without lifetime

    let e1 = Employee {name: "Hakan".to_string(), id: 142, salary: 24_000};

    let e2 = Employee {name: "Aybilge".to_string(), id: 254, salary: 21_000};

    let result = find_higher_salary_owner(e1, e2);

    println!("Higher salary owner: {:?}", result); // Higher salary owner: Employee { name: "Hakan", id: 142, salary: 24000 }

}

#[derive(Debug)]
struct Employee {
    name: String,
    id: u16,
    salary: u16
}

fn find_higher_salary_owner(e1: Employee, e2: Employee) -> Employee {

    if e1.salary > e2.salary {
        e1
    }

    else {

        e2
    }
}
