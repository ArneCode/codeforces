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
    let [n, k] = read_vals_t(s, 0i128);
    let mut map = HashMap::new();
    let mut sum = 0;
    let arr: Vec<i128> = read_arr(s);
    let max_e = if k == 1 { 1 } else if k == -1 {2} else { 50 };
    let mut r: u128 = 0;
    for elt in arr {
        if let Some(amnt) = map.get_mut(&sum) {
            *amnt += 1;
        } else {
            map.insert(sum, 1);
        }
        sum += elt;
        for e in 0..max_e {
            //println!("e: {e}, k: {k}");
            let v = k.pow(e);
            if v.abs() > 10_i128.pow(20) {
                break;
            }
            let searched = sum - v;
            if let Some(amnt) = map.get(&searched) {
                r += amnt;
            }
        }
    }
    result!("{r}");
}
fn main() {
    let mut s = String::new();

    //let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    collections::{HashMap, HashSet},
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
