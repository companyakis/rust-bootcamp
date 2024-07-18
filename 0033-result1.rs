#[derive(Debug)]
enum Branches {
    Ankara,
    Istanbul,
    Izmir,
    Adana,
    Kahramanmaras
}

fn branch_selection(branch: &str) -> Result<Branches, String> {

    match branch {

        "Ankara" => Ok(Branches::Ankara),
        "Istanbul" => Ok(Branches::Istanbul),
        "Izmir" => Ok(Branches::Izmir),
        "Adana" => Ok(Branches::Adana),
        "Kahramanmaras" => Ok(Branches::Kahramanmaras),
        _ => Err("Wrong selection!".to_string())
    }
}

fn selected_branch(branch: Branches) {

    println!("Selected branch is {:?}.", branch);
}

fn main() {

    let b1 = branch_selection("Izmir"); // Selected branch is Izmir.

    match  b1 
    {
        Ok(b) => selected_branch(b),
        Err(e) => println!("Error: {e}"),
    }

    let b2 = branch_selection("Antalya");

    match b2 
    {
        Ok(b) => selected_branch(b),
        Err(e) => println!("Error: {e}"), // Error: Wrong selection!  
        
    }
   
}


