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
    let [n, q] = read_vals_t(s, 0u64);
    let arr: Vec<u64> = read_arr(s);

    let mut new_arr = vec![];
    let mut mappings = vec![];
    let mut reverse_mapping = vec![];

    for (i, v) in arr.iter().enumerate() {
        if v != &0 {
            mappings.push(i);
            new_arr.push(*v);
        }
        reverse_mapping.push(new_arr.len());
    }
    let new_arr = new_arr;

    let mut sums = vec![0];
    let mut sum = 0;
    let mut xors = vec![0];
    let mut xor = 0;

    for v in &new_arr {
        sum += *v;
        xor ^= *v;
        sums.push(sum);
        xors.push(xor);
    }

    for _ in 0..q {
        let [l, r] = read_vals_t(s, 0u64);
        if new_arr.is_empty() || l == r {
            result!("{l} {l}");
            continue;
        }
        let old_l = l;
        let old_r = r;
        let l = l - 1;
        let r = r - 1;
        let mut l = reverse_mapping[l as usize];
        if l >= arr.len() || arr[l] != 0 {
            l -= 1;
        }
        let r = reverse_mapping[r as usize] - 1;
        //println!("l: {l}, r: {r}");

        if l >= r {
            result!("{old_l} {old_l}");
            continue;
        }

        let mut xor = xors[l as usize] ^ xors[r as usize + 1];
        // println!("xor: {xor}, xors: {xors:?}");
        let mut min = r - l + 1;
        let mut min_l = l;
        let mut min_r = r;

        let mut new_l = l;
        loop {
            //println!("new_l: {new_l}");
            let mut loop_xor = xor;
            if r > 0 {
                let mut new_r = r;
                while loop_xor & new_arr[new_r as usize] == new_arr[new_r as usize] {
                    loop_xor ^= new_arr[new_r as usize];

                    if new_r == 0 || new_r == new_l {
                        break;
                    }
                    new_r -= 1;
                    //println!("new_r: {new_r}");
                    let length = (new_r + 1) - new_l;
                    if length < min {
                        min = length;
                        min_l = new_l;
                        min_r = new_r;
                    }
                }
            }
            if xor & new_arr[new_l as usize] != new_arr[new_l as usize] {
                break;
            }
            xor ^= new_arr[new_l as usize];
            if new_l == r {
                break;
            }
            new_l += 1;
            if new_l as usize >= new_arr.len() {
                break;
            }
            let length = (r + 1) - new_l;
            if length < min {
                min = length;
                min_r = r;
                min_l = new_l;
            }
        }
        let min_l = mappings[min_l as usize];
        let min_r = mappings[min_r as usize];
        let l = min_l + 1;
        let r = min_r + 1;
        result!("{l} {r}");
    }
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

