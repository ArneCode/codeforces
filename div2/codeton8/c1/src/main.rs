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
    let [n, x, mut y] = read_vals_t(s, 0u64);
    let mut vs: Vec<u64> = read_arr(s);
    vs.sort();
    //find holes
    let mut r = vs.len() + y as usize - 2;
    let mut prev = vs[0];
    for i in 1..vs.len() {
        let v = vs[i];
        let size = v - prev;
        if size == 2 {
            r += 1;
        }
        if size == 4 && y > 0 {
            r += 2;
            y -= 1;
        }
        prev = v;
    }
    if vs[0] + n - vs.last().unwrap() == 2 {
        r += 1;
    }
    if vs[0] + n - vs.last().unwrap() == 4 && y > 0 {
        r += 2;
        y -= 1;
    }
    let mut prev = vs[0];
    for i in 1..vs.len() {
        if y == 0 {
            break;
        }
        let v = vs[i];
        let size = v - prev;
        if size == 3 {
            r += 1;
            y -= 1;
        }
        if size > 4 {
            let n = (size as usize / 2).min(y as usize);
            r += n;
            y -= n as u64;
        }
        prev = v;
    }
    if vs[0] + n - vs.last().unwrap() >= 3 && y > 0 {
        r += 1;
        y -= 1;
    }
    //r += y as usize;
    r = r.min(n as usize - 2);
    result!("{r}");
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
