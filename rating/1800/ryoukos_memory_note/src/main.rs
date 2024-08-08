macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn solve(s: &mut String) {
    let [n, m] = read_vals_t(s, 0usize);
    let arr: Vec<u64> = read_arr(s);

    let mut pages = vec![vec![]; n];
    let mut r = 0;
    for i in 0..arr.len() {
        let elt = arr[i];
        if i > 0 {
            let p = arr[i - 1];
            if p != elt {
                pages[elt as usize - 1].push(p);
            }
            r += p.abs_diff(elt);
        }
        if i < arr.len() - 1 {
            let n = arr[i + 1];
            if n != elt {
                pages[elt as usize - 1].push(n);
            }
            // r += n.abs_diff(elt);
        }
    }
    //println!("r: {r}");
    let mut best = r;
    for (i, mut others) in pages.into_iter().enumerate() {
        let elt = (i + 1) as u64;
        //println!("elt: {elt}");
        others.sort();
        let mut sum = 0;
        let mut diff_before = 0;
        for o in &others {
            sum += *o;
            diff_before += elt.abs_diff(*o);
        }
        let mut min_new = diff_before;
        let mut before = 0;
        //println!("{others:?}");
        let others_len = others.len();
        for (i, o) in others.into_iter().enumerate() {
            let after = sum - before;
            // println!(
            //     "i: {i}, before: {before}, after: {after}, stuff: {}, start: {}",
            //     ((others_len - 1 - i) as u64) * o,
            //     (i as u64) * o
            // );
            let r = ((i as u64) * o - before) + (after - ((others_len - i) as u64) * o);
            min_new = min_new.min(r);
            before += o;
        }
        let diff = diff_before - min_new;
        //println!("diff_before: {diff_before}, min_new: {min_new}");

        let new_best = r - diff;
        best = best.min(new_best);
    }
    result!("{best}");
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    fmt::Debug,
    io::stdin,
    ops::{Add, Div, Sub},
    str::FromStr,
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

fn read_vals_t<T, const N: usize>(s: &mut String, elt: T) -> [T; N]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
{
    read_vals::<N, T>(s)
}

#[allow(invalid_type_param_default)]
fn read_vals<const N: usize, T = i128>(s: &mut String) -> [T; N]
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
{
    read_arr(s).try_into().unwrap()
}
fn read_arr<T>(s: &mut String) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_line(s);
    s.trim()
        .split(" ")
        .map(|v| v.parse::<T>().unwrap())
        .collect()
}
fn read_line(s: &mut String) {
    s.clear();
    stdin().read_line(s).unwrap();
}
