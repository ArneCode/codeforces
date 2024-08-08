macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn run(even: &[u8], start: Option<u8>, mut b: u32, mut d: u32) -> Option<Vec<u8>> {
    let mut result = vec![];
    if let Some(start) = start {
        if start == 1 {
            if b == 0 {
                return None;
            }
            b -= 1;
        }
        if start == 3 {
            if d == 0 {
                return None;
            }
            d -= 1;
        }
        result.push(start);
    }
    for (i, v) in even.iter().enumerate() {
        if let Some(last) = result.last() {
            if v.abs_diff(*last) != 1 {
                return None;
            }
        }
        result.push(*v);
        if b > 0 {
            b -= 1;
            result.push(1);
        }else if d > 0 {
            if *v == 0 {
                return None;
            }
            d -= 1;
            result.push(3);
        }else if i != even.len() - 1 {
            return None;
        }
    }
    if b > 0 || d > 0 {
        return None;
    }
    Some(result)
}
fn solve(s: &mut String) {
    let [a, b, c, d] = read_vals_t(s, 0u32);
    let mut even = vec![];
    for _ in 0..a {
        even.push(0);
    }
    for _ in 0..c {
        even.push(2);
    }
    if let Some(result) = run(&even, None, b, d) {
        println!("YES");
        for v in result {
            print!("{v} ");
        }
        println!();
        return;
    }
    if let Some(result) = run(&even, Some(1), b, d) {
        println!("YES");
        for v in result {
            print!("{v} ");
        }
        println!();
        return;
    }
    if let Some(result) = run(&even, Some(3), b, d) {
        println!("YES");
        for v in result {
            print!("{v} ");
        }
        println!();
        return;
    }
    println!("NO");
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
