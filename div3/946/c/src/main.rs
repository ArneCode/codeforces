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
    let arr:Vec<u64> = read_arr(s);

    let mut b1: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    let mut b2: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
    let mut b3: HashMap<(u64, u64), Vec<u64>> = HashMap::new();

    for i in 0..(arr.len()-2) {
        let p3 = (arr[i],arr[i+1]);
        let p2 = (arr[i],arr[i+2]);
        let p1 = (arr[i+1],arr[i+2]);

        let v1 = arr[i];
        let v2 = arr[i+1];
        let v3 = arr[i+2];

        if let Some(l) = b1.get_mut(&p1) {
            l.push(v1);
        }else{
            b1.insert(p1, vec![v1]);
        }

        if let Some(l) = b2.get_mut(&p2) {
            l.push(v2);
        }else{
            b2.insert(p2, vec![v2]);
        }

        if let Some(l) = b3.get_mut(&p3) {
            l.push(v3);
        }else{
            b3.insert(p3, vec![v3]);
        }
    }

    let mut r = 0;

    for vs in b1.values_mut() {
        vs.sort();

        let mut l = vec![];

        let mut i = 0;
        while i < vs.len() {
            let v = vs[i];
            let mut len = 0;
            while i < vs.len() && vs[i] == v {
                len += 1;
                i += 1;
            }
            l.push(len);
        }

        let tot: u64 = l.iter().sum();

        for len in l {
            let others = tot - len;
            r += len*others;
        }
    }
    for vs in b2.values_mut() {
        vs.sort();

        let mut l = vec![];

        let mut i = 0;
        while i < vs.len() {
            let v = vs[i];
            let mut len = 0;
            while i < vs.len() && vs[i] == v {
                len += 1;
                i += 1;
            }
            l.push(len);
        }

        let tot: u64 = l.iter().sum();

        for len in l {
            let others = tot - len;
            r += len*others;
        }
    }
    for vs in b3.values_mut() {
        vs.sort();

        let mut l = vec![];

        let mut i = 0;
        while i < vs.len() {
            let v = vs[i];
            let mut len = 0;
            while i < vs.len() && vs[i] == v {
                len += 1;
                i += 1;
            }
            l.push(len);
        }

        let tot: u64 = l.iter().sum();

        for len in l {
            let others = tot - len;
            r += len*others;
        }
    }
    r/=2;
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
    collections::HashMap, fmt::Debug, io::stdin, ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub}, str::FromStr
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
