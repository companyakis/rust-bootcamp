mod accounting;
mod sales;

pub use crate::accounting::accounting_department;

pub use crate::sales::sales_department;


fn main() { 

    accounting_department::accounting_dep_info(); // Welcome to Accounting Department!

    sales_department::sales_department_warning(); // Please, check the inventory!

    //sales_department::sales_department_hidden_warning_message(); // Error => private function
}




