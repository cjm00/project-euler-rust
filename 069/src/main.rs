extern crate oiler;

const K: usize = 1_000_000;


fn solve() -> usize {
    let mut primes: Vec<usize> = oiler::load_primes();

    let mut index: usize = 0;
    let mut n: usize = 1;
    let mut phi_n: usize = 1;
    let mut ratio: f64 = 0.0;
    let mut largest: usize = 0;
    let mut largest_ratio: f64 = 0.0;

    while n <= K {
        if ratio > largest_ratio {
            largest = n;
            largest_ratio = ratio;
        }
        n *= primes[index];
        phi_n *= primes[index] - 1;
        ratio = n as f64 / phi_n as f64;

        index += 1;
    }

return largest
}

fn main() {
    println!("{}", solve());
}
//
