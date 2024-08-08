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
    read_line(s);
    let s = s.trim();
    let mut divs = vec![];
    let mut d = 1;
    while d * d < n + 5 {
        if n % d == 0 {
            divs.push(d);
            divs.push(n / d);
        }
        d += 1;
    }
    divs.sort();
    let bytes = s.as_bytes();
    let mut r = u64::MAX;
    'outer: for seg_len in divs {
        //println!("len: {seg_len}");
        let n_segs = n / seg_len;
        'a: for a in 0..n_segs.min(2) {
            let mut diffs = 0;
            for i in 0..n_segs {
                for pos in 0..seg_len {
                    let x = i * seg_len + pos;
                    if bytes[(a * seg_len + pos) as usize] != bytes[x as usize] {
                        // println!(
                        //     "diff at {pos}, x: {x}, bytes[pos]: {}, bytes[x], {}",
                        //     bytes[pos as usize], bytes[x as usize]
                        // );
                        diffs += 1;
                        if diffs > 1 {
                            continue 'a;
                        }
                    }
                }
            }
            result!("{seg_len}");
            return;
        }

        // r = r.min(*seg_len);
    }
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
