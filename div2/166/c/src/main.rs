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
    let [n, m] = read_vals_t(s, 0usize);

    let ps: Vec<i64> = read_arr(s);
    let ts: Vec<i64> = read_arr(s);

    let mut np = 0;
    let mut nt = 0;

    let mut r = 0;

    let mut s = None;

    let mut is_p = vec![false; ps.len()];
    for i in 0..ps.len() {
        let p = ps[i];
        let t = ts[i];

        if (p > t && np <= n) || nt == m {
            is_p[i] = true;
            np += 1;
            r += p;

            s = Some(i);
        } else {
            nt += 1;
            r += t;

            // if nt == m && s.is_none() {
            //     s = Some(i);
            // }
        }
    }

    let mut last_p = None;
    for i in (0..ps.len()).rev() {
        if is_p[i] {
            if last_p.is_none() {
                last_p = Some(i);
            }
        }
    }
    let last_p = last_p.unwrap();
    let mut last_f = None;

    let mut rs = vec![];
    for i in (0..ps.len()).rev() {
        if is_p[i] {
            if ps[i] <= ts[i] {
                last_f = Some(i);
            }
            let tot = r - ps[i];
            rs.push(tot);
        } else {
            let mut tot = r - ts[i];
            if let Some(last_f) = last_f {
                tot -= ps[last_f];
                tot += ts[last_f];
            } else {
                tot -= ps[last_p];
                tot += ts[last_p];
            }
            rs.push(tot);
        }
    }
    for v in rs.iter().rev() {
        print!("{v} ");
    }
    println!();

    // let s = s.unwrap();

    // for i in 0..ps.len() {
    //     if is_p[i] {
    //         let tot = r - ps[i];
    //         print!("{tot} ");
    //     }else{
    //         let mut tot = r - ts[i];
    //         if is_p[s] {
    //             tot -= ps[s];
    //             tot += ts[s];
    //             print!("{tot} ");
    //         }else{
    //             tot -= ts[s];
    //             tot += ps[s];
    //             print!("{tot} ");
    //         }
    //     }
    // }
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    for _ in 0..n_t {
        solve(&mut s);
    }
}
use std::{
    fmt::Debug,  io::stdin, ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub}, path::is_separator, str::FromStr
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
fn pow_mod<T>(mut b: T, mut e: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy,
{
    let mut r = T::from(1);
    while e > 0u64.into() {
        if e % 2u64.into() == 1u64.into() {
            r *= b;
            r %= mod_n;
        }
        b *= b;
        b %= mod_n;
        e >>= 1;
    }
    r
}
fn div_mod<T>(zähler: T, nenner: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>,
{
    (zähler * pow_mod(nenner, mod_n - 2u64.into(), mod_n)) % mod_n
}
fn fac_mod<T>(n: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    let mut r = 1u64.into();
    let mut i = 1u64.into();
    while i <= n {
        r *= i;
        r %= mod_n;
        i += 1u64.into();
    }
    r
}
fn binomial_koef_mod<T>(n: T, k: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    div_mod(
        fac_mod(n, mod_n),
        (fac_mod(n - k, mod_n) * fac_mod(k, mod_n)) % mod_n,
        mod_n,
    )
}
