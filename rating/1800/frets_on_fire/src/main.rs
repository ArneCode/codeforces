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
    let mut s_arr: Vec<u128> = read_arr(s);

    s_arr.sort();

    let mut dists = vec![];
    
    let mut prev_v = s_arr[0];
    for v in s_arr.iter().skip(1) {
        dists.push(v  - prev_v);
        prev_v = *v;
    }
    dists.push(u128::MAX);
    dists.sort();

    let mut sum = 0;
    let mut dp = vec![0];
    for dist in &dists {
        sum += dist;
        dp.push(sum);
    }

    //println!("dists: {dists:?}, dp: {dp:?}");

    let [q] = read_vals_t(s, 0u32);
    for _ in 0..q {
        let [l, r] = read_vals_t(s, 0u128);
        let diff = r - l;
        let x = binary_edge(|i| {
            dists[i] > diff
        }, 0, dists.len() - 1);
        // if x == 0 {
        //     print!("1 ");
        //     continue;
        // }
        //println!("x: {x}, dp: {}, rest: {}", dp[x], (diff + 1)*(s_arr.len() as u128 - x as u128));
        let r = dp[x] + (diff + 1)*(s_arr.len() as u128 - x as u128);
        print!("{r} ");
    }
    println!()
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
