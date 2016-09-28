fn main() {
    const UPPER_LIMIT: u64 = 1001*1001;
    let mut multiplier: u64 = 0;
    let mut counter: u64 = 0;
    let mut sum: u64 = 0;
    let mut current: u64 = 1;
    while current <= UPPER_LIMIT {
        sum += current;
        if counter % 4 == 0 {multiplier += 1};
        counter += 1;
        current += 2 * multiplier;
    }
    println!("Sum: {}", sum);
}
