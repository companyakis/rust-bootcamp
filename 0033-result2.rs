#[derive(Debug)]
struct SellDecision {
  
    price: f32
}

fn check_price(currency: &SellDecision) -> Result<String, String> {

    if currency.price < 15.0 {
      
        Err("Wait! Price should be bigger than 15.0 ₺...".to_string())
    } 
    else {
      
        Ok("We can sell now!".to_string())
    }
 }

fn main() {

    let c1 = SellDecision { price: 18.54};

    println!("{:?}", check_price(&c1)); // Ok("We can sell now!")

    let c2 = SellDecision { price: 13.24};

    println!("{:?}", check_price(&c2)); // Err("Wait! Price should be bigger than 15.0 ₺...")
   
}
