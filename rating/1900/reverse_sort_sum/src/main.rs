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
    read_line(s);
    let arr: Vec<u64> = read_arr(s);

    let sum: u64 = arr.iter().sum();
    let n_ones = sum / arr.len() as u64;

    if arr.len() == 1 {
        result!("{n_ones}");
        return;
    }

    let mut ones_seen = 0;
    let mut ones_after = vec![0; arr.len() + 2];

    let mut result = vec![0; arr.len()];

    let mut n_left = n_ones;
    for i in (0..arr.len()).rev() {
        let times_sorted = (arr.len() - i) as u64;
        let buildup_end = if times_sorted > n_left {
            //println!("min: {i}, max: {}", arr.len() - 1);
            binary_edge(
                |x| {
                    let ones_in_range = ones_seen - ones_after[x + 1];
                    ones_in_range + n_left <= (x - i) as u64
                },
                i,
                arr.len() - 1,
            )
        } else {
            arr.len()
        };
        let ones_in_range = ones_seen - ones_after[buildup_end + 1];
        let expected = (n_left + ones_in_range).min(times_sorted);
        if arr[i] > expected || (i == 0 && n_left > 0) {
            //println!("expected: {expected}, got: {}", arr[i]);
            result[i] = 1;
            ones_seen += 1;
            n_left -= 1;
        } else {
            result[i] = 0;
        }
        ones_after[i] = ones_seen;
    }
    for v in result {
        print!("{v} ");
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
