use std::{collections::HashSet, io::stdin};

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    let n_t = s.trim().parse().unwrap();

    for _ in 0..n_t {
        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr: Vec<i128> = s
            .trim()
            .split(" ")
            .map(|n| n.trim().parse::<i128>().unwrap())
            .collect();

        let l = arr[0];
        let p = arr[1];

        let max_n = (l + p) + 10;
        //println!("max_n")
        let mut n = 1;
        let mut ks = HashSet::new();
        if (p + l) % 2 != 0 {
            println!("0");
            continue;
        }
        let mut a = true;
        let mut b = true;
        while n*n < max_n && (a || b) {
            if (l - p) % (2 * n) == 0 && a {
                let k = (l - p) / (2 * n) + 1;
                //println!("k = {k} at n = {n} (2n+1)");
                let np = l - 2 * n * k + 2 * n;
                if p == np {
                    if k < p {
                        // a = false;
                    } else {
                        ks.insert(k);
                    }
                } else {
                    println!("didn't insert, np: {np}");
                }

                let k = n + 1;
                let n = (l - p) / (2 * n);
                let np = l - 2 * n * k + 2 * n;
                //println!("k = {k} at n = {n} (2n + 1), np: {np}");

                if p == np {
                    if k < p {
                    } else {
                        ks.insert(k);
                    }
                }
            }
            if (p + l - 2) % (2 * n) == 0 && b {
                let k = (p + l - 2) / (2 * n) + 1;
                //println!("k = {k} at n = {n} (2n)");
                let np = 2 * n * k - l - 2 * (n - 1);
                if p == np {
                    if k < p {
                        b = false;
                    } else {
                        ks.insert(k);
                    }
                } else {
                    println!("didn't insert, np: {np}");
                }
                let k = n + 1;
                let n = (p + l -2) / (2*n);
                let np = 2 * n * k - l - 2 * (n - 1);

                //println!("k = {k} at n = {n} (2n), np: {np}");

                if p == np {
                    if k < p {
                    } else {
                        ks.insert(k);
                    }
                }
            }
            n += 1;
        }
        println!("{}", ks.len());
        //dbg!(ks);
    }
}
