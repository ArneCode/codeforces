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
    let [_, pos] = read_vals_t(s, 0usize);
    let pos = pos - 1;
    let arr: Vec<i128> = read_arr(s);

    let mut barrs = vec![0; arr.len()];
    let mut gains = vec![0; arr.len()];

    let mut barrier = 0;
    let mut gain = 0;

    for i in (0..pos).rev() {
        gain += arr[i];
        barrier = barrier.max(-gain);

        barrs[i] = barrier;
        gains[i] = gain;
    }

    let mut barrier = 0;
    let mut gain = 0;

    for i in (pos + 1)..arr.len() {
        gain += arr[i];
        barrier = barrier.max(-gain);

        barrs[i] = barrier;
        gains[i] = gain;
    }

    let mut l = pos;
    let mut r = pos;
    let mut gainsl = 0;
    let mut gainsr = 0;

    let mut h = arr[pos];

    //println!("barrs: {barrs:?}");
    //println!("gains: {gains:?}");

    'outer: while l > 0 && r + 1 < arr.len() {
        //println!("l: {l}, r: {r}");
        for i in (r + 1)..arr.len() {
            //println!("r, i: {i}");
            let barrier = barrs[i];
            let gain = gains[i];
            //println!("barrier: {barrier}, gainsr: {gainsr}, h: {h}");
            if barrier + gainsr > h {
                r = i - 1;
                break;
            }
            if gain > gainsr {
                h += gain - gainsr;
                gainsr += gain - gainsr;
                r = i;
                gains[i] = 0;
                continue 'outer;
            }
            if i == arr.len() - 1 {
                break 'outer;
            }
        }
        for i in (0..(l)).rev() {
            //println!("l, i: {i}, h: {h}");
            let barrier = barrs[i];
            let gain = gains[i];

            if barrier + gainsl > h {
                //l = i + 1;
                break;
            }
            if gain > gainsl {
                h += gain - gainsl;
                gainsl += gain - gainsl;
                l = i;
                gains[i] = 0;
                continue 'outer;
            }
            if i == 0 {
                break 'outer;
            }
        }
        println!("NO");
        return;
    }
    println!("YES");
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    for i in 0..n_t {
        // if i == 9 && n_t == 11630 {
        //     let [_, pos] = read_vals_t(&mut s, 0usize);
        //     let pos = pos - 1;
        //     let arr: Vec<i128> = read_arr(&mut s);

        //     println!("arr:{arr:?},pos: {pos}");
        // }
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
