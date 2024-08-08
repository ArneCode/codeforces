use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t: usize = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();
        let mut n: u128 = s.trim().parse().unwrap();
        let mut result: i128 = 0;

        if n >= 30 {
            result = n as i128 / 15 - 1;
            n = n % 15 + 15;
        }
        result += vec![
            0, 1, 2, 1, 2, 3, 1, 2, 3, 2, 1, 2, 2, 2, 3, 1, 2, 3, 2, 3, 2, 2, 3, 3, 3, 2, 3, 3, 3,
            4,
        ][n as usize];
        println!("{result}");
    }
}
