fn main() {

    let mut years: Vec<u16> = Vec::new();

    years.push(2012);
    years.push(2014);
    years.push(2016);
    years.push(2018);
    years.push(2020);

    println!("Vector: {:?}", years); // Vector: [2012, 2014, 2016, 2018, 2020]

    years.pop();

    println!("Vector: {:?}", years); // Vector: [2012, 2014, 2016, 2018] 

    years.remove(1);

    println!("Vector: {:?}", years); // Vector: [2012, 2016, 2018]  
}


