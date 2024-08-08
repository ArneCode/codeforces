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
    let [n] = read_vals_t(s, 0u64);
    let mut largest = 0;
    for o in 0..n {
        println!("? {o} {o} {largest} {largest}");
        stdout().flush().unwrap();
        read_line(s);
        let a = s.trim();
        if a == ">" {
            largest = o;
        }
    }
    let largest = largest;
    let mut is = vec![];
    let mut best_i = 0;
    for i in 1..n {
        println!("? {i} {largest} {best_i} {largest}");
        stdout().flush().unwrap();
        read_line(s);
        let a = s.trim();
        if a == ">" {
            best_i = i;
            is.clear();
            is.push(i);
        }else if a == "=" {
            is.push(i);
        }
    }
    // find smallest i
    for i in is {
        println!("? {i} {i} {best_i} {best_i}");
        stdout().flush().unwrap();
        read_line(s);
        let a = s.trim();
        if a == "<" {
            best_i = i;
        }
    }
    println!("! {largest} {best_i}");
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
    io::{stdin, stdout, Write},
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
