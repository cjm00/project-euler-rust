use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

pub fn modular_exp(b: usize, exp: usize, modulus: usize) -> usize {
    if modulus == 1 || (b % modulus == 0) { return 0 }
    let mut result = 1;
    let mut base = b;
    let mut exponent = exp;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    return result;
}

pub fn load_primes<T: FromIterator<usize>>() -> T {
    let mut data = String::new();
    let mut f = File::open(r#"I:\Data\primes1\primes1.txt"#).expect("Couldn't open file");
    f.read_to_string(&mut data).expect("Unable to read file to string");
    let mut set: T = data.split_whitespace()
                     .map(|s| s.parse::<usize>().unwrap())
                     .collect();
    return set
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
    }

#[test]
fn gcd_test() -> (){
    assert!(gcd(1, 7) == 1);
    assert!(gcd(3, 5) == 1);
    assert!(gcd(12, 60) == 12);
    assert!(gcd(12, 90) == 6);
    assert!(gcd(0, 5) == 5);
    assert!(gcd(5, 0) == 5);
}
