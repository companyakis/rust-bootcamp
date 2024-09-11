fn main() {

    let man_fintech = Managers::FinTech("Mustafa Buyukdereli".to_string());

    let man_sales = Managers::Sales("Bilge Çoksatan".to_string());

    let man_finance = Managers::Finance("Hakan Konmaz".to_string());

    let man_hr = Managers::HR("Aybüke Karataş".to_string());

    let man_operations = Managers::Operations("Aygün Kaplan".to_string());

    let man_audit = Managers::Audit("Kutluk Yapıcı".to_string());

    println!("What we have: {:?}", man_fintech); // What we have: FinTech("Mustafa Buyukdereli")

    match man_fintech {

        Managers::FinTech(data) => println!("FinTech manager: {}", data), // FinTech manager: Mustafa Buyukdereli
        _ => (),

    }

}

#[derive(Debug)]
enum Managers {
    FinTech(String),
    Sales(String),
    Finance(String),
    HR(String),
    Operations(String),
    Audit(String)
}

