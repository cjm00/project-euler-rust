use std::fs::File;
use std::io::Read;
use std::collections::hash_set::HashSet;


fn load_primes() -> HashSet<u64> {
    let mut data = String::new();
    let mut f = File::open(r#"I:\Data\primes1\primes1.txt"#).expect("Couldn't open file");
    f.read_to_string(&mut data).expect("Unable to read file to string");
    let set: HashSet<u64> = data.split_whitespace()
                                .map(|s| s.parse::<u64>().unwrap())
                                .filter(|s| *s < 1_000_000)
                                .collect();
    return set
}

fn main() {
    let primes = load_primes();
    let mut small_primes = primes.iter().cloned().filter(|x| *x < 5_000).collect::<Vec<u64>>();
    small_primes.sort();
    for k in 500..1000 {
    let example = small_primes.windows(k)
                              .map(|s| s.iter().fold(0, |sum, x| sum + x))
                              .filter(|s| primes.contains(&s))
                              .last();
    println!("{:?} {}", example, k);
}
}
