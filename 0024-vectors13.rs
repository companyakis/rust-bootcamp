fn main() {

    let mut departments: Vec<Department> = Vec::new();

    let dep_fintech = Department {name: "FinTech".to_string(), head: "Mustafa Büyükdereli".to_string(), yearly_budget_usd: 180_000, number_of_people: 8};

    let dep_finance: Department = Department {name: "Finance".to_string(), head: "Aygün Kaplan".to_string(), yearly_budget_usd: 66_000, number_of_people: 2};

    departments.push(dep_fintech);
    departments.push(dep_finance);

    for i in &departments {
        println!("Department head: {:?} and department yearly budget : $ {:?}", i.head, i.yearly_budget_usd);
    }

}

struct Department {
    name: String,
    head: String,
    yearly_budget_usd: u32,
    number_of_people: u8
}

// Department head: "Mustafa Büyükdereli" and department yearly budget : $ 180000
// Department head: "Aygün Kaplan" and department yearly budget : $ 66000


