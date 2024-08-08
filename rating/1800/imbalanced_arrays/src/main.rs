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
    let mut a: Vec<u32> = read_arr(s);

    let mut b = vec![0; a.len()];

    let mut idxs: Vec<usize> = (0..a.len()).collect();

    idxs.sort_by_key(|i| a[*i]);

    let mut n_less = vec![0;a.len()];

    let mut v: i32 = 1;
    
    let mut n_diff = 1;
    let mut i = 0;
    let mut prev = 0;
    while i < idxs.len() {
        let idx = idxs[i];
        let v = a[idx];

        if i > 0 {
            if v != prev {
                n_diff += 1;
            }
        }
        n_less[idx] = n_diff - 1;
        prev = v;
        i += 1;
    }

    // if n_diff == 1 && a[0] == 0 {
    //     println!("YES");
    //     for i in 0..a.len() {
    //         print!("-{} ", i + 1);
    //     }
    //     println!();
    //     return;
    // }


    // let mut sp = binary_edge(|i| {
    //     n_less[idxs[i]] >= n_diff / 2
    // }, 0, idxs.len());

    // if n_less[idxs[sp]] < n_diff / 2 {
    //     println!("No");
    //     return;
    // }

    let mut sp = binary_edge(|sp: usize|{
        let n_pos = a.len() - sp;
        a[idxs[sp]] >= n_pos as u32
    }, 0,idxs.len());

    let n_pos = a.len() - sp;
    //println!("n_pos: {n_pos}");

    let mut sm: i32 = sp as i32 - 1;

    let mut np = 0;
    let mut nm = 0;

    while sp < a.len() {
        let elt = a[idxs[sp]];

        //let n_pos = a.len().div_ceil(2);

        if n_pos as u32 + nm as u32 > elt {
            //println!("elt: {elt}, v: {v}");
            println!("NO");
            return;
        }
        let n_minus = elt as usize - n_pos -nm;

        //println!("elt: {elt}, n_minus: {n_minus}");
        for _ in 0..n_minus {
            if sm < 0 {
                println!("nO");
                return;
            }
            let ngr = a.len() - sp;
            if ngr as u32 != a[idxs[sm as usize]] {
                println!("No");
                return;
            }
            b[idxs[sm as usize]] = -v;
            v += 1;
            sm -= 1;
            nm += 1;
        }

        b[idxs[sp]] = v;

        v += 1;
        sp += 1;
        np += 1;
    }
    for sm in (0..=sm).rev() {
        let elt = a[idxs[sm as usize]];
        let ngr = a.len() - sp;
        if ngr as u32 != elt {
            //println!("elt: {elt}, sp: {sp}, b: {b:?}");
            println!("no");
            return;
        }
        b[idxs[sm as usize]] = -v;
        v += 1;
    }
    println!("yEs");
    for v in b {
        print!("{v} ");
    }
    println!();

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
