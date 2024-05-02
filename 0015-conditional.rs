fn main() {

    let n1 = -1254;

    num_check(n1);

    let n2 = 10000005;

    num_check(n2);

    let n3 = 10110;

    num_check(n3);

}

fn num_check(num: i128) {
    
    if num % 5 == 0 { println!("Number {num} is divisible by 5."); }

    else if num % 10 == 0 { println!("Number {num} is divisible by 10."); }

    else { println!("Number {num} is NOT divisible by 5 or 10."); }
}


/*
Number -1254 is NOT divisible by 5 or 10.
Number 10000005 is divisible by 5.
Number 10110 is divisible by 5.   

*/

