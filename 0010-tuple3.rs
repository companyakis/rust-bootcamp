fn main() {

    let mut founder_info: (&str, &str, u8, bool) = ("Mustafa", "Büyükdereli", 1, true); 

    founder_info.2 = 100; //let's update founder id

    println!("Founder info: {founder_info:?}") //Founder info: ("Mustafa", "Büyükdereli", 100, true)

}
