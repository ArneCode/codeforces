macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn get_clockw(p_n: i128, dist: i128, max_n: i128) -> i128 {
    (p_n + dist) % max_n
}
fn get_counter(p_n: i128, dist: i128, max_n: i128) -> i128 {
    let r = p_n - dist;
    if r < 0 {
        r + max_n
    } else {
        r
    }
}
fn solve(s: &mut String) {
    let [n, m, x] = read_vals(s);
    let n = n as usize;
    let mut vs = vec![false; n as usize];
    vs[x as usize - 1] = true;
    for _ in 0..m {
        let arr: Vec<String> = read_arr(s);
        let d: i128 = arr[0].trim().parse().unwrap();
        let r = match arr[1].as_str() {
            "0" => 1,  //clockw
            "1" => -1, //counterclock
            "?" => 0,
            _ => panic!(),
        };
        let mut new_vs = vec![false; n];
        for (i, v) in vs.into_iter().enumerate() {
            if !v {
                continue;
            }
            if r == 0 || r == 1 {
                let rp = get_clockw(i as i128, d, n as i128);
                new_vs[usize::try_from(rp).unwrap()] = true;
            }
            if r == 0 || r == -1 {
                let rp = get_counter(i as i128, d, n as i128);
                new_vs[usize::try_from(rp).expect(&format!("rp: {rp}, i: {i}, d: {d}, m: {n}"))] =
                    true;
            }
        }
        vs = new_vs;
    }
    let n = vs.iter().filter(|v| **v).count();
    result!("{n}");
    for (i, v) in vs.into_iter().enumerate() {
        if v {
            print!("{} ", i + 1);
        }
    }
    println!();
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    for _ in 0..n_t {
        solve(&mut s);
    }
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
