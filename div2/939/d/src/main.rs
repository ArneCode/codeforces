macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn rec(l: u64, r: u64, ops: &mut Vec<(u64, u64)>) {
    let (l, r) = (l.min(r), r.max(l));
    if l == r {
        ops.push((l, r));
        return;
    }
    rec(l, r - 1, ops);
    // 3 3 3 0
    for i in (l + 1)..=(r - 1) {
        ops.push((i, i));
    }
    // 3 0 0 0
    for l in (l + 1)..r {
        rec(l, r - 1, ops);
        for i in (l + 1)..r {
            ops.push((i, i));
        }
    }
    ops.push((l, r));
}
fn solve(s: &mut String) {
    let [n] = read_vals_t(s, 0u64);
    let arr: Vec<u64> = read_arr(s);

    let mut max = 0;
    let mut max_counter = 0;

    for counter in 0..=((1 << n) - 1) {
        let mut v = 0;
        let mut n_0 = 0;
        for i in 0..n {
            let bit = 1 << i;

            if bit & counter > 0 {
                if n_0 > 0 {
                    v += n_0 * n_0;
                    n_0 = 0;
                }
                v += arr[i as usize];
            } else {
                n_0 += 1;
            }
        }
        v += n_0 * n_0;
        if v >= max {
            max = v;
            max_counter = counter;
        }
    }

    let mut ranges = vec![];
    let mut s = 0;

    let mut n0: u64 = 0;
    for i in 0..n {
        let bit = 1 << i;

        if bit & max_counter > 0 {
            if n0 > 0 {
                ranges.push((i - n0, i - 1));
                s += n0 * n0;
                n0 = 0;
            }
            s += arr[i as usize];
        } else {
            n0 += 1;
        }
    }

    if n0 > 0 {
        ranges.push((n - n0, n - 1));
        s += n0 * n0;
    }

    let mut ops = vec![];

    for (l, r) in ranges {
        for i in l..=r {
            if arr[i as usize] != 0 {
                ops.push((i, i));
            }
        }
        rec(l, r, &mut ops);
    }
    let out = &mut BufWriter::new(stdout());
    writeln!(out, "{s} {}", ops.len()).unwrap();

    for (l, r) in ops {
        writeln!(out, "{} {}", l + 1, r + 1).unwrap();
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
    fmt::Debug,
    io::{stdin, stdout, BufWriter, Write},
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
