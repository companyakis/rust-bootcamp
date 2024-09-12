fn main() {

    let daily_tl_sales_min_targets: Vec<u32> = vec![100_000; 365];

    println!("Weekly sales targets: {:?}", &daily_tl_sales_min_targets[0..7]); // Weekly sales targets: [100000, 100000, 100000, 100000, 100000, 100000, 100000]

    // box => from stack to heap

    let daily_tl_sales_min_targets_2= Box::new([100_000; 365]);

    println!("Weekly sales targets: {:?}", daily_tl_sales_min_targets_2.len());


}


