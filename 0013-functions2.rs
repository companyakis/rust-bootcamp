fn main() {

    let radius = 12 as f32;

    let perimeter = circle_perimeter(radius);

    let area = circle_area(radius);

    println!("Perimeter: {perimeter} and area: {area}"); // Perimeter: 75.36 and area: 452.16

}

fn circle_perimeter(radius: f32) -> f32 {

    return 2.0 * 3.14 * radius;
}

fn circle_area(radius: f32) -> f32 {

    return 3.14 * radius * radius;
}


