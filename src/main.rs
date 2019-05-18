use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(3 == args.len());
    let n = args[1].parse::<usize>()
                    .expect("arg must be a positive int");
    let m = args[2].parse::<usize>()
                    .expect("arg must be a positive int");
    println!("{}", gcd(n,m));
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    // Euclids Greatest Common Divisor Algo
    // given two integers
    // iteratively, 
    // set the larger int to 
    // the remainder of the larger
    // divided by the smaller
    // when you get a remainder of zero
    // return the divisor 
    // (the smaller int)
    // that's the gcd
    //
    // - Euclid, 300 BC
    // earth's first software dev
    if n == 0 || m == 0 {
        panic!("cannot compute the greatest common divisor for zero")
    }
    loop {
        if n < m {
            // for simplicity ensure n > m each iteration
            // if this is not the case 
            // swap values for n and m
            // done here without making an extra variable
            // just to be silly fancy
            n = n + m;
            m = n - m;
            n = n - m;
        }
        n = n % m;
        if n == 0 {
            break
        }
    }
    m
}
