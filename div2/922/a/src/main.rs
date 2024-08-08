use std::{io::stdin, vec};

fn bytes(mut n: u64) -> Vec<bool> {
    let mut a = vec![];
    for _ in 0..64 {
        a.push(n % 2 == 1);
        n >>= 1;
    }
    return a.into_iter().rev().collect();
}
fn solve(s: u64, l: u64, mut r: u64) -> u64 {
    let diff = s ^ l;
    let diff_b = bytes(diff);
    // let r_b = bytes(r);
    let s_b = bytes(s);
    let l_b = bytes(l);

    let mut rl = 0;
    let mut rs = 0;
    for i in 0..64 {
        rs <<= 1;
        rl <<= 1;
        let d_bit = diff_b[i];
        //let r_bit = r_b[i];
        let s_bit = s_b[i];
        let l_bit = l_b[i];

        let i_u = 63 - i;
        let c_v = 2_u64.pow(i_u as u32);
        if d_bit {
            if r >= c_v {
                // println!("a");
                let c = if rl > 0 {
                    rs += 1;
                    !s_bit
                } else {
                    rl += 1;
                    !l_bit
                };
                if c {
                    r -= c_v;
                }
            } else {
                rs += u64::from(s_bit);
                rl += u64::from(l_bit);
            }
        }
    }
    rl.abs_diff(rs)
}
fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();

        stdin().read_line(&mut s).unwrap();

        let arr: Vec<u64> = s.split(" ").map(|n| n.trim().parse::<u64>().unwrap()).collect();

        let a = arr[0];
        let b = arr[1];
        let r = arr[2];

        let result = solve(a, b, r).min(solve(b, a, r));
        println!("{result}");
    }
}
