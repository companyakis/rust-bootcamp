fn main() {

    let man_fintech = Managers::FinTech("Mustafa Buyukdereli".to_string());

    // without "match"

    if let Managers::FinTech(data) = man_fintech {

        println!("FinTech manager: {}", data); // FinTech manager: Mustafa Buyukdereli
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

