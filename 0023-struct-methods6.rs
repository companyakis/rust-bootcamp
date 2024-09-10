use std::collections::HashMap;

fn main() {

    let students_results = HashMap::from([
        (Student::new_student("Mustafa".to_string(), 440), 98),
        (Student::new_student("Ayhan".to_string(), 417), 94),
        
    ]);

    println!("Students results: {:?}", students_results);

    for (key, value) in &students_results {
        println!("{:?} -> {:?}", key.name, value);
    }

    /*
    "Ayhan" -> 94  
    "Mustafa" -> 98    
    */

}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Student {
    name: String,
    id: u16
}

impl Student {
    fn new_student(name: String, id: u16) -> Student {
        Student {name: name, id: id}
    }
}



