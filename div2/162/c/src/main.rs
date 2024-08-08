use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let n_q = s.trim().split(" ").nth(1).unwrap().trim().parse().unwrap();

        s.clear();

        stdin().read_line(&mut s).unwrap();

        let arr: Vec<u128> = s.split(" ").map(|n| n.parse::<u128>().unwrap()).collect();
        for _ in 0..n_q {}
    }
}
