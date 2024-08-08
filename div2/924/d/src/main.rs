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
            .map(|n| n.trim().parse::<i128>().unwrap())
            .collect();

        let n = arr[0];
        let b = arr[1];
        let x = arr[2];

        s.clear();
        stdin().read_line(&mut s).unwrap();

        let mut arr: Vec<i128> = s
            .trim()
            .split(" ")
            .map(|n| n.trim().parse::<i128>().unwrap())
            .collect();

        arr.sort();

        let mut cs = vec![(0, 0)];
        let mut i = 0;
        while i < arr.len() {
            let v = arr[i];
            let mut amnt = 0;
            while i < arr.len() && arr[i] == v {
                amnt += 1;
                i += 1;
            }
            cs.push((v, amnt));
        }

        let strength = |ki: usize| -> i128 {
            let mut result = 0;
            let k = cs[ki].0;

            for i in 0..=ki {
                let (v, amnt) = cs[i];
                result += (v * (v - 1) / 2) * amnt * b;
                //println!("v = {v}, amnt = {amnt}, result = {result} 1");
            }
            for i in (ki + 1)..(cs.len()) {
                let (v, amnt) = cs[i];
                let height = v / k;
                result += (k * (k - 1) / 2) * height * height * amnt * b;
                let top_w = v % k;
                result += (top_w * (top_w - 1) / 2 + top_w * height * (k - 1)) * amnt * b;
                //println!("v = {v}, amnt = {amnt}, result = {result} 2");
            }
            result -= (k - 1) * x;
            //dbg!(&k, &result);
            result
        };
        if cs.len() < 1000 {
            let result = (1..cs.len()).map(strength).max().unwrap();
            println!("{result}");
            continue;
        }else{
            println!("no");
        }
    }
}
