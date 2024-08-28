fn main() {

    let ft = Departments::FinTech(5);
    
    if let Departments::FinTech(x) = ft {
        
        println!("{x}"); //5
    }

}


#[derive(Debug)]

// let's hold number of employees
enum Departments {
    FinTech(u8),
    Sales(u8),
    Finance(u8),
    Accounting(u8),
    Marketing(u8),
    IS(u8),
    InternalAudit(u8)
}
