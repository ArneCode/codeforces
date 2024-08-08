use std::{
    io::stdin,
    ops::{Add, Div, Sub},
};

/// Returns the x of the first true Value
fn binary_edge<F, X>(f: F, min_x: X, max_x: X) -> X
where
    F: Fn(X) -> bool,
    X: PartialOrd
        + PartialEq
        + Add<Output = X>
        + Sub<Output = X>
        + Div<Output = X>
        + Copy
        + From<u8>,
{
    if f(min_x) {
        return min_x;
    }
    let mut a = min_x;
    let mut b = max_x;
    loop {
        let mid = (a + b) / 2.into();
        let mv = f(mid);

        if mv {
            b = mid;
        } else {
            a = mid;
        }

        if b - a == 1.into() {
            break b;
        }
    }
}
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
            .map(|n| n.parse::<i128>().unwrap())
            .collect();

        let n = arr[0];
        let c = arr[1];

        s.clear();
        stdin().read_line(&mut s).unwrap();

        let arr: Vec<i128> = s
            .trim()
            .split(" ")
            .map(|n| n.parse::<i128>().unwrap())
            .collect();

        let mut r = (c + 1) * (c + 2) / 2;

        let mut even = 0;
        let mut uneven = 0;

        for v in &arr {
            r -= c - v + 1;
            if v % 2 == 0 {
                even += 1;
            } else {
                uneven += 1;
            }
            if v % 2 == 0 {
                let n = if *v <= c { v / 2 } else { c - *v / 2 } + 1;
                let n_taken = even;
                r -= n - n_taken;
            } else {
                let n = if *v <= c {
                    v / 2
                } else {
                    c - (*v + 1) / 2
                } + 1;

                let n_taken = uneven;
                r -= n - n_taken;
            }
        }
        println!("{r}");
    }
}
