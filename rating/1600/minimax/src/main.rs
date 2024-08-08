macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn aa(ls: &Vec<(char, u64)>) -> Option<Vec<char>> {
    let (a, mut a_amnt) = ls[0];
    let mut b_i = 1;
    let (mut b, mut b_amnt) = ls[b_i];
    let mut result = vec![];
    result.push(a);
    result.push(a);
    a_amnt -= 2;
    loop {
        if a_amnt == 0 {
            break;
        }
        if b_amnt == 0 {
            b_i += 1;
            if b_i >= ls.len() {
                return None;
            }
            (b, b_amnt) = ls[b_i];
        }
        result.push(b);
        b_amnt -= 1;
        result.push(a);
        a_amnt -= 1;
        if a_amnt == 0 {
            break;
        }
    }
    loop {
        if b_amnt == 0 {
            b_i += 1;
            if b_i >= ls.len() {
                break;
            }
            (b, b_amnt) = ls[b_i];
        }
        result.push(b);
        b_amnt -= 1;
    }
    return Some(result);
}
fn aba(ls: &Vec<(char, u64)>) -> Option<Vec<char>> {
    let (a, mut a_amnt) = ls[0];
    let (b, mut b_amnt) = ls[1];
    let mut result = vec![];
    if ls.len() == 2 {
        return None;
    }
    let (c, mut c_amnt) = ls[2];
    result.push(a);
    result.push(b);
    for _ in 1..a_amnt {
        result.push(a);
    }
    result.push(c);
    for _ in 1..b_amnt {
        result.push(b);
    }
    for _ in 1..c_amnt {
        result.push(c);
    }
    for i in 3..ls.len() {
        let (x, x_amnt) = ls[i];
        for _ in 0..x_amnt {
            result.push(x);
        }
    }
    return Some(result);
}
fn abb(ls: &Vec<(char, u64)>) -> Option<Vec<char>> {
    let (a, mut a_amnt) = ls[0];
    let (b, mut b_amnt) = ls[1];
    let mut result = vec![];
    result.push(a);
    for _ in 0..b_amnt {
        result.push(b);
    }
    for _ in 1..a_amnt {
        result.push(a);
    }
    for i in 2..ls.len() {
        let (x, x_amnt) = ls[i];
        for _ in 0..x_amnt {
            result.push(x);
        }
    }
    return Some(result);
}
fn solve(s: &mut String) {
    read_line(s);
    let chars: Vec<char> = s.trim().chars().collect();
    let mut ls = vec![0u64; 26];
    for c in chars {
        ls[c as usize - 'a' as usize] += 1;
    }
    for (i, l) in ls.iter().enumerate() {
        if *l == 1 {
            ls[i] -= 1;
            print!("{}", ('a' as u8 + i as u8) as char);
            for (i, amnt) in ls.iter().enumerate() {
                for _ in 0..*amnt {
                    print!("{}", ('a' as u8 + i as u8) as char);
                }
            }
            println!();
            return;
        }
    }
    let ls: Vec<(char, u64)> = ls
        .into_iter()
        .enumerate()
        .filter(|(i, amnt)| *amnt > 0)
        .map(|(i, a)| (('a' as u8 + i as u8) as char, a))
        .collect();
    if ls.len() == 1 {
        let (x, x_amnt) = ls[0];
        for _ in 0..x_amnt {
            print!("{x}");
        }
        println!();
        return;
    }
    if let Some(result) = aa(&ls) {
        for c in result {
            print!("{c}");
        }
        println!();
        return;
    }
    if let Some(result) = aba(&ls) {
        for c in result {
            print!("{c}");
        }
        println!();
        return;
    }
    if let Some(result) = abb(&ls) {
        for c in result {
            print!("{c}");
        }
        println!();
        return;
    }
    panic!()
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
