use std::io;
 
fn main() -> io::Result<()> {
    let t = read_number();
    let mut primes = vec![];
    for _i in 0..t {
        let n = read_number();
        println!("{}", get_result(n, &mut primes));
    }
 
    Ok(())
}
 
fn read_number() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error!");
    input.trim().parse::<i64>().expect("parse error!")
}
 
fn get_result(n: i64, primes:&mut Vec<i64>) -> i64 {
    let mut current = n;
    let mut prime_factor = 1i64;
    let mut prime = 2i64;    
 
    for d in primes {
        while current % d == 0 {
            current /= prime;
            prime_factor = prime;
        }
    }
 
    while current > 1
    {
        for i in prime_factor + 1..=current {
            if is_prime(i) {
                prime_factor = i;
                primes.push(i);                
                break;
            }
        }
    }
 
    prime_factor
}
 
fn is_prime(n: i64) -> bool {
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
 
    true
}