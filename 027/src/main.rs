use std::fs::File;
use std::io::Read;
use std::collections::hash_set::HashSet;


fn quadratic (n: i64, a: i64, b: i64) -> i64 {
    n.pow(2) + a*n + b
}

fn prime_sequence_length(a: i64, b: i64, primes: &HashSet<i64>) -> i64 {
    if !primes.contains(&quadratic(0, a, b)) {return 0};
    let mut n: i64 = 0;
    while primes.contains(&quadratic(n, a, b)) {n += 1;};
    return n;

}

fn load_primes() -> HashSet<i64> {
    let mut data = String::new();
    let mut f = File::open(r#"I:\Data\primes1\primes1.txt"#).expect("Couldn't open file");
    f.read_to_string(&mut data).expect("Unable to read file to string");
    let set: HashSet<i64> = data.split_whitespace()
                                .map(|s| s.parse::<i64>().unwrap())
                                .collect();
    return set
}

fn main() {
    let primes_set = load_primes();
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut max_length: i64 = 0;
    let mut current_length: i64 = 0;

    for x in -999..1000 {
        for y in -1000..1001{
            current_length = prime_sequence_length(x, y, &primes_set);
            if current_length > max_length {
                max_length = current_length;
                a = x;
                b = y;
            }
        }
    }
    println!("Largest generators: a = {}, b = {} with sequence length {}", a, b, max_length);
    println!("Their product: {}", a*b);

}
