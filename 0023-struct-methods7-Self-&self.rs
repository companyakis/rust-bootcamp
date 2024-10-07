// Self and &self

fn main() {

    let emp1 = Employee {name: "Ayhan".to_string(), department: "HR".to_string(), id: 245};

    emp1.print_employee_info(); // Employee info => Ayhan - HR - 245

    let emp2 = Employee::new_employee("Hakan".to_string(), "Finance".to_string(), 412);

    emp2.print_employee_info(); // Employee info => Hakan - Finance - 412

}

struct Employee {
    name: String,
    department: String,
    id: u16
}

impl Employee {

    // &self
    fn print_employee_info(&self) {
        println!("Employee info => {} - {} - {}", self.name, self.department, self.id)
    }

    // Self => like constructor?!
    fn new_employee(name: String, department: String, id: u16) -> Self {
        Self {name, department, id}
    }

}
