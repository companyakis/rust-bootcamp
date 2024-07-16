fn main() {

    let partner_mustafa = Employee {
        name: "Mustafa Büyükdereli".to_string(),
        id: 1,
        department: Departments::FinTech
    };

    partner_mustafa.employee_info(); // Name: Mustafa Büyükdereli, ID: 1 and department: FinTech

}

#[derive(Debug)]
enum Departments {
    FinTech, Sales, Accounting, Marketing, Finance, Operations, HR, IS, Audit
}

struct Employee {
    name: String,
    id: u16,
    department: Departments
}

impl Employee {

    fn employee_info(&self) {

        println!("Name: {}, ID: {} and department: {:?}", self.name, self.id, self.department);
    }
    
}
