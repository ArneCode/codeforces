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
    read_line(s);
    let s = s.trim();
    let chars: Vec<char> = s.chars().collect();

    let mut result = 0;
    let mut prev_a = None;
    let mut prev_b = None;
    for i in 0..s.len() {
        let c = chars[i];
        if c == 'A' {
            if let Some(prev_a) = prev_a {
                result += prev_a + 1;
                if let Some(prev_b) = prev_b {
                    if prev_b < prev_a {
                        result -= 1;
                    }
                }
            }
        }else{
            if let Some(prev_b) = prev_b {
                result += prev_b + 1;
                if let Some(prev_a) = prev_a {
                    if prev_a < prev_b {
                        result -= 1;
                    }
                }
            }
        }
        if c == 'A' {
            prev_a = Some(i);
        }else{
            prev_b = Some(i);
        }
    }
    result!("{result}");
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
