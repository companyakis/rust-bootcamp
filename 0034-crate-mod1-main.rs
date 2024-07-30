fn main() { 

    accounting_department::accounting_dep_info(); // Welcome to Accounting Department!

    sales_department::sales_department_warning(); // Please, check the inventory!

    //sales_department::sales_department_hidden_warning_message(); // Error => private function
}

mod accounting_department {

    pub fn accounting_dep_info() {

        println!("Welcome to Accounting Department!");
    }
}

mod sales_department {

    pub fn sales_department_warning() {

        println!("Please, check the inventory!");
    }

    // private

    fn sales_department_hidden_warning_message() {

        println!("Sales are decreasing!");
    }
}
    
