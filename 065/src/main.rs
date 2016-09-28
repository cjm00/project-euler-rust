fn main() {
    // http://oeis.org/A007676
    let conv_100 = "6963524437876961749120273824619538346438023188214475670667";
    println!("{}", conv_100.chars()
                            .map(|s| s.to_digit(10).unwrap())
                            .fold(0, |acc, x| acc + x));
}
