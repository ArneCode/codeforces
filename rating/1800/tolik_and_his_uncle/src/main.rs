macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn r(rows: u64, cols: u64, x_d: u64, mut result: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if cols == 1 {
        let mut l = 1;
        let mut r = rows;
        loop {
            result.push((l, 1 + x_d));
            l += 1;
            if l > r {
                return result;
            }
            result.push((r, 1 + x_d));
            r -= 1;
            if r < l {
                return result;
            }
        }
    }
    let mut left = 1;
    let mut right = rows;
    while right >= 1 {
        result.push((left, 1 + x_d));
        left += 1;
        result.push((right, cols + x_d));
        right -= 1;
    }
    if cols == 2 {
        return result;
    }
    return r(rows, cols - 2, x_d + 1, result);
}
fn solve(s: &mut String) {
    let [n, m] = read_vals_t(s, 0u64);
    let result = r(n, m, 0, Vec::with_capacity((n * m) as usize));
    let out = &mut BufWriter::new(stdout());
    for (a, b) in result {
        writeln!(out, "{a} {b}").ok();
    }
}
fn main() {
    let mut s = String::new();

    solve(&mut s);
}
use std::{
    fmt::Debug,
    io::{stdin, stdout, BufWriter, Write},
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
