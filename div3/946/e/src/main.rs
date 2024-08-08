macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn dfs(
    months: &Vec<(u64, u64)>,
    curr_money: u64,
    curr_h: u64,
    i: usize,
    max: &mut u64,
    max_poss: &Vec<u64>,
    x: u64,
) {
    let mut money = curr_money + x;
    if i == 0 {
        money = 0;
    }
    //println!("i: {i}, money: {curr_money}, curr_h: {curr_h}");

    let max_happ = curr_h + max_poss[i];
    if max_happ < *max {
        //println!("max_happ: {max_happ}");
        return;
    }

    if i >= months.len() {
        *max = curr_h;
        return;
    }

    let (c, h) = months[i];

    if c <= money {
        dfs(months, money - c, curr_h + h, i + 1, max, max_poss, x);
    }
    dfs(months, money, curr_h, i + 1, max, max_poss, x);
}
fn solve(s: &mut String, p: bool) {
    let [m, x] = read_vals_t(s, 0u64);

    let mut months = vec![];

    for _ in 0..m {
        let [c, h] = read_vals_t(s, 0u64);
        months.push((c, h));
    }

    let mut max_poss = vec![0];

    let mut sum = 0;
    for (c, h) in months.iter().rev() {
        sum += h;
        max_poss.push(sum);
    }
    max_poss.reverse();

    let mut max = 0;

    dfs(&months, 0, 0, 0, &mut max, &max_poss, x);
    // if p {
        result!("{max}");
    // }
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);

    // if n_t == 1000 {
    //     for _ in 0..103 {
    //         solve(&mut s, false);
    //     }
    //     for _ in 0..25 {
    //         read_line(&mut s);
    //         println!("{s}");
    //     }
    //     return;
    // }

    for _ in 0..n_t {
        solve(&mut s, true);
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
