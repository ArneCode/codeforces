fn solve(s: &mut String) {
    read_line(s);

    let mut arr = read_arr::<u128>(s);

    let [n_q] = read_vals(s);

    let mut p_s = vec![0];

    let mut sum = 0;

    for i in 1..arr.len() {
        if i == 1 {
            sum += 1;
        } else {
            let v = arr[i];
            let pv = arr[i - 1];
            let ppv = arr[i - 2];

            if v.abs_diff(pv) < pv.abs_diff(ppv) {
                sum += 1;
            }else{
                sum += v.abs_diff(pv);
            }
        }
        p_s.push(sum);
    }

    arr.reverse();

    let mut p_s_r = vec![0];

    let mut sum = 0;

    for i in 1..arr.len() {
        if i == 1 {
            sum += 1;
        } else {
            let v = arr[i];
            let pv = arr[i - 1];
            let ppv = arr[i - 2];

            if v.abs_diff(pv) < pv.abs_diff(ppv) {
                sum += 1;
            }else{
                sum += v.abs_diff(pv);
            }
        }
        p_s_r.push(sum);
    }

    p_s_r.reverse();


    for _ in 0..n_q {
        let [x, y] = read_vals_t(s, 0usize);

        let r = if y > x {
            p_s[y - 1] - p_s[x - 1]
        } else {
            p_s_r[y - 1] - p_s_r[x - 1]
        };
        println!("{r}");
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
