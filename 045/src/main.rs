use std::collections::hash_set::HashSet;

fn triangular(n: u64) -> u64 {(n * (n - 1)) / 2}
fn pentagonal(n: u64) -> u64 {(n * (3*n - 1)) / 2}
fn hexagonal(n: u64) -> u64 {n * (2*n - 1)}

fn main() {
    let mut pent_set = HashSet::<u64>::new();
    let mut hex_set = HashSet::<u64>::new();
    let mut index = 144; // start after the first example given
    pent_set.insert(pentagonal(index));
    hex_set.insert(hexagonal(index));

    while !pent_set.contains(&triangular(index)) || !hex_set.contains(&triangular(index)) {
        index += 1;
        pent_set.insert(pentagonal(index));
        hex_set.insert(hexagonal(index));
    }

    println!("First number: {}", triangular(index));
}
