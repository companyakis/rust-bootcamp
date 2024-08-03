fn area_selection(area: &str) {

    match area {

        "ai" => println!("AI is very important area for our company."),

        "web3" => println!("Web3 is very important area for our company."),

        "algotrade" => panic!("We are not ready yet! We need time!"),

        _ => println!("Unrelated area for us! Sorry..."),
    }
}

fn main() {

    area_selection("ai"); // AI is very important area for our company.

    area_selection("accounting"); // Unrelated area for us! Sorry...

    // thread 'main' panicked at src\main.rs:9:24:
    area_selection("algotrade"); // We are not ready yet! We need time!        

}

