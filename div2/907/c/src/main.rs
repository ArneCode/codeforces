use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        stdin().read_line(&mut s).unwrap();

        s.clear();
        stdin().read_line(&mut s).unwrap();

        let mut arr: Vec<u128> = s
            .trim()
            .split(" ")
            .map(|n| n.trim().parse::<u128>().unwrap())
            .collect();

        arr.sort();

        let sum: u128 = arr.iter().sum();

        let mut n_c = 0;
        let mut c_s = 0;

        let sum_end = sum / 2;
        let sum_start = sum - sum_end;

        if sum_end > 0 {
            for v in arr.iter().rev() {
                n_c += 1;
                c_s += v;

                if c_s >= sum_end {
                    break;
                }
            }
        }
        let result = sum_start + n_c;
        println!("{result}");
    }
}
