fn main() {

    let founder = String::from("Mustafa Kemal Atatürk");

    println!("{founder}");

    let mut me = "Mustafa ".to_string();

    me.push('B'); //char

    me.push_str("üyükderelii");

    println!("{me}");

    me.pop();

    println!("{me}");

}

/*
Mustafa Kemal Atatürk
Mustafa Büyükderelii
Mustafa Büyükdereli 
*/

