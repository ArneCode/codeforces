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
    let m = 998244353;
    read_line(s);

    let a: Vec<u64> = read_arr(s);

    let len = a.len();

    let max = *a.iter().max().unwrap();
    let mut max_count = 0;

    let mut one_less_count = 0;
    for v in &a {
        if *v == max {
            max_count += 1;
        }
        if *v == max - 1 {
            one_less_count += 1;
        }
    }

    if max_count > 1 {
        let r = fac_mod(a.len() as u64, m);

        result!("{r}");
        return;
    }
    if one_less_count == 0 {
        result!("0");
        return;
    }

    let l = one_less_count;
    let r = a.len() as u64 - l - 1;

    let mut res = 0;

    let mut bs = vec![1];
    let mut b = 1;

    let mut b = 1;
    for i in (1..=r).rev() {
        b *= i;
        b %= m;
        bs.push(b);
    }

    let mut a = fac_mod(l - 1, m);

    for s in 0..=r {
        //let a = fac_mod(l + s, m);
        a*= l + s;
        a %= m;

        let b = bs.pop().unwrap();
        res += a * b;
        res %= m;
    }
    let n_fac = fac_mod(len as u64, m);

    //println!("n_fac: {n_fac}, res: {res}");
    let r = if res > n_fac {
        n_fac + m - res
    } else {
        n_fac - res
    };
    //let r = (n_fac - res) % m;
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
