fn main() {
    let city: Cities = Cities::IstanbulEuropeanSide(34);

    let area_code: u16 = match city {
        Cities::Ankara(6) => 312,
        Cities::Izmir(35) => 232,
        Cities::IstanbulEuropeanSide(34) => 212,
        Cities::IstanbulAnatolianSide(34) => 216,
        Cities::Adana(1) => 322,
        Cities::Kahramanmaras(46) => 344,
        _ => 0,
    };

    println!("Plate code: {:?} and phone area code: {:?}", city, area_code);
}

#[derive(Debug)]
enum Cities {
    Ankara(u16),
    Izmir(u16),
    IstanbulEuropeanSide(u16),
    IstanbulAnatolianSide(u16),
    Adana(u16),
    Kahramanmaras(u16),
}
