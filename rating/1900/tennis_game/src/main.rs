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
    //let [n] = read_vals_t(s, 0usize);
    read_line(s);

    let a: Vec<u8> = read_arr(s);

    let mut n1 = 0;
    let mut sum1 = vec![0];
    let mut n2 = 0;
    let mut sum2 = vec![0];

    for v in &a {
        if *v == 1 {
            n1 += 1;
        } else {
            n2 += 1;
        }
        sum1.push(n1);
        sum2.push(n2);
    }
    let mut r = vec![];

    'outer: for t in 1..=n1.max(n2) {
        let mut w1 = 0;
        let mut w2 = 0;

        let mut i = 0;

        while i < a.len() {
            // println!("i: {i}");
            let set_end = if (i + t - 1).min(a.len() - 1) == (i + 2 * t + 2).min(a.len() - 1) {
                a.len() - 1
            } else {
                binary_edge(
                    |x| {
                        let n1 = sum1[x + 1] - sum1[i];
                        let n2 = sum2[x + 1] - sum2[i];

                        n1 >= t || n2 >= t
                    },
                    (i + t - 1).min(a.len() - 1),
                    (i + 2 * t + 2).min(a.len() - 1),
                )
            };
            // println!("t: {t}, i: {i}, set_end: {set_end}");

            let n1 = sum1[set_end + 1] - sum1[i];
            let n2 = sum2[set_end + 1] - sum2[i];

            // if !(n1 == t || n2 == t) {
            //     println!("no, n1: {n1}, n2: {n2}");
            //     continue 'outer;
            // }
            if n1 == t {
                w1 += 1;
            } else if n2 == t {
                w2 += 1;
            } else {
                continue 'outer;
            }

            i = set_end + 1;
        }
        if w1 == w2 {
            continue;
        }
        if w1 > w2 && *a.last().unwrap() == 2 {
            continue;
        }
        if w2 > w1 && *a.last().unwrap() == 1 {
            continue;
        }
        r.push((w1.max(w2), t));
    }
    // println!("sdaf");
    r.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    println!("{}", r.len());
    for (s, t) in r {
        result!("{s} {t}");
    }
}
fn main() {
    let mut s = String::new();

    //let [n_t] = read_vals(&mut s);

    //for _ in 0..n_t {
    solve(&mut s);
    //}
}
use std::{
    fmt::Debug,
    io::stdin,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub},
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
