fn main() {

    let mut counter: u8 = 11;
    
    loop {
        
        counter += 3;
        
        if counter == 101 {
            
            break
        }
        
        println!("Counter: {counter}")
    }
    
    println!("Final counter: {counter}")
}

// Counter: 14
// Counter: 17
// Counter: 20
// Counter: 23
// Counter: 26
// Counter: 29
// Counter: 32
// Counter: 35
// Counter: 38
// Counter: 41
// Counter: 44
// Counter: 47
// Counter: 50
// Counter: 53
// Counter: 56
// Counter: 59
// Counter: 62
// Counter: 65
// Counter: 68
// Counter: 71
// Counter: 74
// Counter: 77
// Counter: 80
// Counter: 83
// Counter: 86
// Counter: 89
// Counter: 92
// Counter: 95
// Counter: 98
// Final counter: 101
