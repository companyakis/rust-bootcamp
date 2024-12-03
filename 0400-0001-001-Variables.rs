fn main() {
    // rust infers the type of x
    let x = 22.0 / 7.0;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.1412;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 2024;
    println!("{}", x);
}
