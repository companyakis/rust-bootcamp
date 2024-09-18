fn main() {

    let monthly_sales_usd: [u32; 12] = [55_000, 47_000, 88_000, 92_000, 65_000, 48_700, 54_000, 98_000, 45_400, 52_000, 68_400, 76_980];

    let mut sales_iter = monthly_sales_usd.iter();

    let max_monthly_sales: Option<&u32> = sales_iter.max();

    println!("Maximum monthly sales: {}", max_monthly_sales.unwrap()); // Maximum monthly sales: 98000
 
}


