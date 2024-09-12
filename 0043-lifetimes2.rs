fn main() {

}
// Error! => help: consider introducing a named lifetime parameter

// Assume they don't have equal lengths... Keep it simple now:)

fn find_longest(x: &str, y: &str, z: &str) -> &str {

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

