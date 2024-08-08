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
    let a: Vec<i64> = read_arr(s);

    //let af: Vec<f64> = a.iter().map(|v| *v as f64).collect();

    let mut info = Vec::with_capacity(a.len());

    let mut sum = vec![0.0];
    let mut s = 0;

    for elt in &a {
        s += elt;
        sum.push(s as f64);
    }
    sum.push(s as f64);

    for i in 0..(a.len()) {
        let iu = i as i64;
        let seg_len = if i == 0 { 1 } else { iu & -iu };

        //println!("i: {i:b}, seg_len: {seg_len:b}");

        let mut min = f64::MAX;
        let mut min_pos = 0;

        for j in 1..=(seg_len as usize) {
            if i + j > a.len() {
                //println!("i: {i}, j: {j}, min: {min}, min_pos: {min_pos}");
                break;
            }
            let sum = sum[i + j] - sum[i];

            let v = (sum as f64) / (j as f64);

            if v < min {
                min = v;
                min_pos = i + j - 1;
            }
        }
        info.push((seg_len as usize, min_pos));
    }

    let mut i = 0;
    let mut r = vec![];

    while i < a.len() {
        //println!("a");
        let mut j = i;

        let mut min = f64::MAX;
        let mut min_pos = 0;
        while j < a.len() {
            //println!("b");

            let (seg_len, min_seg_pos) = info[j];


            let sum = sum[min_seg_pos + 1] - sum[i];

            let v = sum / ((min_seg_pos + 1 - i) as f64);
            println!("j: {j}: {j:b}, seg_len: {seg_len}: {seg_len:b}, v: {v}");
            if v < min {
                println!("updating min to {v}, min_pos: {min_pos}");
                min = v;
                min_pos = min_seg_pos;
            }

            j += seg_len;
        }

        println!("min_pos is {min_pos}");

        for _ in i..=min_pos {
            r.push(min);
        }
        i = min_pos + 1;
    }

    if r.len() != a.len() {
        panic!();
    }


    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::with_capacity(8192, handle);
    for v in r {
        writeln!(writer, "{v}").unwrap();
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
    io::{self, stdin, BufWriter, Write},
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
