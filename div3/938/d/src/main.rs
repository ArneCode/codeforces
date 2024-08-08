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
    let [n, m, k] = read_vals_t(s, 0u64);
    let arr_a: Vec<usize> = read_arr(s);
    let arr_b: Vec<usize> = read_arr(s);

    let mut amnts_a = HashMap::new(); //vec![0; 10usize.pow(6) + 1];
    let mut amnts_b = HashMap::new(); //vec![0; 10usize.pow(6) + 1];
    for elt in arr_b {
        //amnts_b[elt as usize] += 1;
        if let Some(amnt) = amnts_b.get_mut(&(elt as usize)) {
            *amnt += 1;
        } else {
            amnts_b.insert(elt, 1u64);
        }
    }
    for elt in &arr_a[0..(m as usize)] {
        // amnts_a[*elt as usize] += 1;
        if let Some(amnt) = amnts_a.get_mut(elt) {
            *amnt += 1;
        } else {
            amnts_a.insert(*elt, 1);
        }
    }
    let mut mat = 0;
    // for i in 0..amnts_b.len() {
    //     let a = amnts_a[i];
    //     let b = amnts_b[i];

    //     mat += a.min(b);
    // }
    for (elt, b) in &amnts_b {
        let a = if let Some(a) = amnts_a.get(&elt) {
            *a
        } else {
            continue;
        };
        mat += a.min(*b);
    }
    //println!("mat: {mat}");
    let mut r = 0;
    if mat >= k {
        r += 1;
    }
    for i in 0..((n - m) as usize) {
        {
            let elt_remov = arr_a[i];
            let a = amnts_a.get_mut(&elt_remov).unwrap();
            if let Some(b) = amnts_b.get(&elt_remov) {
                let p_c = (*a).min(*b);
                *a -= 1;
                let a_c = (*a).min(*b);
                mat -= p_c - a_c;
            }
        }

        {
            let elt_new = arr_a[i + m as usize];
            let a = if let Some(a) = amnts_a.get_mut(&elt_new) {
                a
            } else {
                amnts_a.insert(elt_new, 0);
                amnts_a.get_mut(&elt_new).unwrap()
            };
            if let Some(b) = amnts_b.get(&elt_new) {
                let p_c = (*a).min(*b);
                *a += 1;
                let a_c = (*a).min(*b);
                mat += a_c - p_c;
            }
        }
        // let p_c = amnts_a[elt_new].min(amnts_b[elt_new]);
        // amnts_a[elt_new] += 1;
        // let a_c = amnts_a[elt_new].min(amnts_b[elt_new]);
        // mat += a_c - p_c;

        if mat >= k {
            r += 1;
        }
    }
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
    collections::{HashMap, HashSet},
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
