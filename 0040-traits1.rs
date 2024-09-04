fn main() {

    let men_info: Men = Men {count: 242, average_salary: 37500.0, average_age: 38.5};

    println!("Report: {}", men_info.general_report()); // Report: Statistiscs on men in our city => Number: 242 - Average Salary: 37500 - Average Age: 38.5

    let women_info: Women = Women {count: 255, average_salary: 27230.0, average_age: 33.4};

    println!("Report: {}", women_info.general_report()); // Report: Statistiscs on men in our city => Number: 255 - Average Salary: 27230 - Average Age: 33.4
}

pub trait ReportingTools {
    fn general_report(&self) -> String;
}

pub struct Men {
    count: u32,
    average_salary: f32,
    average_age: f32,
}

pub struct Women {
    count: u32,
    average_salary: f32,
    average_age: f32,
}

impl ReportingTools for Men {
    fn general_report(&self) -> String {
        format!(
            "Statistiscs on men in our city => Number: {} - Average Salary: {} - Average Age: {}",
            self.count, self.average_salary, self.average_age
        )
    }
}

impl ReportingTools for Women {
    fn general_report(&self) -> String {
        format!(
            "Statistiscs on men in our city => Number: {} - Average Salary: {} - Average Age: {}",
            self.count, self.average_salary, self.average_age
        )
    }
}
