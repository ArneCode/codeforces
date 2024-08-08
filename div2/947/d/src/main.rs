macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn opt(verts: &Vec<Vec<usize>>, i: usize, seen: &mut Vec<bool>) -> (u64, u64) {
    // n_all, n_up
    seen[i] = true;

    let mut n = 0;

    let mut max_depth = 0;
    for child in &verts[i] {
        if seen[*child] {
            continue;
        }
        let (child_n, child_depth) = opt(verts, *child, seen);
        n += child_n + 1;
        n += child_depth;
        max_depth = max_depth.max(child_depth);
    }
    n -= max_depth;
    (n, max_depth + 1)
}
fn path(
    verts: &Vec<Vec<usize>>,
    a: usize,
    b: usize,
    visited: &mut Vec<bool>,
) -> Option<Vec<usize>> {
    visited[a] = true;
    if a == b {
        return Some(vec![b]);
    }
    for child in &verts[a] {
        if visited[*child] {
            continue;
        }
        if let Some(mut p) = path(verts, *child, b, visited) {
            p.push(a);
            return Some(p);
        }
    }
    None
}
fn solve(s: &mut String) {
    let [n] = read_vals(s);
    let [a, b] = read_vals_t(s, 0usize);
    let a = a - 1;

    let b = b - 1;

    let mut verts = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let [x, y] = read_vals_t(s, 0usize);
        let x = x - 1;
        let y = y - 1;

        verts[x].push(y);
        verts[y].push(x);
    }

    let mut visited = vec![false; n];
    let path = path(&verts, a, b, &mut visited).unwrap();

    // if path.len() % 2 == 1 {
    let mid = path.len() / 2;

    // }

    let mut r = mid as u64;

    let mut seen = vec![false; n];
    let (n_all, n_up) = opt(&verts, path[mid], &mut seen);
    r+= n_all;
    result!("{r}");
    // println!("{n_all}, n_up: {n_up}");
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
