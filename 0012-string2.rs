fn main() {

    let founder = String::from("Mustafa Kemal Atatürk");

    for c in founder.chars() {
        print!("{c} "); //M u s t a f a   K e m a l   A t a t ü r k
    }

    for b in founder.bytes() {
        print!("{b} "); //77 117 115 116 97 102 97 32 75 101 109 97 108 32 65 116 97 116 195 188 114 107
    }

    //index position

    for (i, c) in founder.char_indices() {
        println!("{i} -> {c}");
    }

}

/*
0 -> M 
1 -> u 
2 -> s 
3 -> t 
4 -> a 
5 -> f 
6 -> a 
7 ->   
8 -> K 
9 -> e 
10 -> m
11 -> a
12 -> l
13 ->  
14 -> A
15 -> t
16 -> a
17 -> t
18 -> ü
20 -> r
21 -> k
*/

