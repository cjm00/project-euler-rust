extern crate oiler;

const TARGET: usize = 250250;
const K: usize = 250;
const ANSWER_CUTOFF: u64 = 1_0000_0000_0000_0000;

fn initialize_vec() -> Vec<usize> {
    let vec_target: Vec<usize> = (1..TARGET+1).map(|n| oiler::modular_exp(n, n, K))
                            .collect();
    assert_eq!(TARGET, vec_target.len());
    return vec_target;
}

fn carry_residues(old_residues: &Vec<u64>, current: usize) -> Vec<u64> {
    let mut new_residues = old_residues.clone();
    for i in 0..K {
        new_residues[i] += old_residues[(current + i) % K];
        new_residues[i] = new_residues[i] % ANSWER_CUTOFF;
    }
    return new_residues;

}

fn solve() -> u64 {
    let vec_target = initialize_vec();
    let mut residues = vec![0; K as usize];
    residues[0] = 1;
    for i in 0..TARGET {
        residues = carry_residues(&residues, vec_target[i]);
    }
    return (residues[0] - 1) % ANSWER_CUTOFF;
}

fn main() {
    println!("{}", solve());
}
