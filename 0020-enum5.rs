fn main() {

    println!("Barcelona info => {:?}", Teams::Barcelona("Spain".to_string(), 18));

    println!("Galatasaray info => {:?}", Teams::Galatasaray("Turkiye".to_string(), 16));
    
    /*
    Barcelona info => Barcelona("Spain", 18)
    Galatasaray info => Galatasaray("Turkiye", 16)
    */

}

#[derive(Debug)]
enum Teams {

    Barcelona(String,u8),
    Galatasaray(String, u8)
}
