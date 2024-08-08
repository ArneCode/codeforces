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
    let [n, q] = read_vals_t(s, 0u64);
    let arr: Vec<u64> = read_arr(s);
    let max = *arr.iter().max().unwrap();
    let mut deque = VecDeque::from(arr);

    let mut rs = vec![];
    loop {
        let a = deque.pop_front().unwrap();
        let b = deque.pop_front().unwrap();
        if a == max {
            deque.push_front(b);
            deque.push_front(a);
            break;
        }
        rs.push((a, b));
        if a > b {
            deque.push_front(a);
            deque.push_back(b);
        } else {
            deque.push_front(b);
            deque.push_back(a);
        }
    }
    let stable = Vec::from(deque);
    let a = stable[0];
    let stable = stable[1..].to_vec();
    //println!("stable: {stable:?}");
    for _ in 0..q {
        let [m] = read_vals_t(s, 0u128);
        let m = m - 1;
        if m < rs.len() as u128 {
            let (a, b) = rs[m as usize];
            result!("{a} {b}");
            continue;
        }
        let m = m - rs.len() as u128;
        let m = m % stable.len() as u128;
        let b = stable[m as usize];
        result!("{a} {b}");
    }
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    collections::VecDeque, fmt::Debug, io::stdin, num::Saturating, ops::{Add, Div, Sub}, str::FromStr
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
