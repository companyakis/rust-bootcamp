fn main() {

    let w1 = "ceteris paribus";
    let w2 = "lorem ipsum";
    let w3 = "carpe diem";

    let result = find_longest(w1, w2, w3);

    println!("The longest: {:?}", result); // The longest: "ceteris paribus"

}

// assume the lengths are not equal! Keep it simple now...

fn find_longest<'a>(x: & 'a str, y: & 'a str, z: & 'a str) -> & 'a str {

    if (x.len() > y.len()) & (x.len() > z.len()) {
        x
    }

    else if (y.len() > x.len()) & (y.len() > z.len()) {
        y
    }

    else {
        z
    }
}

