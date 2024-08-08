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
    let mut arr: Vec<usize> = read_arr(s);
    for v in arr.iter_mut() {
        *v -= 1;
    }
    let mut free = vec![true; arr.len()];
    let mut n_over = 0;
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, t) in arr.iter().enumerate() {
        free[*t] = false;
        if let Some(many) = map.get_mut(t) {
            many.push(i);
            n_over += 1;
        } else {
            map.insert(*t, vec![i]);
        }
    }
    let mut frees = vec![];
    for (i, v) in free.iter().enumerate() {
        if *v {
            frees.push(i);
        }
    }
    let mut score = arr.len() - n_over;
    let mut result = arr;
    if n_over == 0 {
    } else if n_over == 1 {
        //println!("test");
        dbg!("test");
        let free = frees[0];
        for (t, many) in map.into_iter() {
            if many.len() == 2 {
                for m in many {
                    if m == free {
                        continue;
                    } else {
                        result[m] = free;
                        break;
                    }
                }
            }
        }
    } else {
        let mut over: HashSet<usize> = HashSet::new();
        for (t, mut many) in map.into_iter() {
            many.pop();
            over.extend(many.iter());
        }
        // println!("frees: {frees:?}, over: {over:?}");
        let mut rot = vec![];
        let mut no_rot_frees = vec![];
        for free in frees {
            if over.contains(&free) {
                rot.push(free);
                over.remove(&free);
            } else {
                no_rot_frees.push(free);
            }
        }
        if rot.len() == 1 {
            let rot = rot[0];
            let o = *over.iter().next().unwrap();
            over.take(&o);
            let free = no_rot_frees.pop().unwrap();
            result[rot] = free;
            result[o] = rot;
        } else {
            // println!("rot: {rot:?}, over: {over:?}");
            for i in 0..rot.len() {
                result[rot[i]] = rot[(i + 1) % rot.len()];
            }
        }
        for o in over {
            result[o] = no_rot_frees.pop().unwrap();
        }
    }
    result!("{score}");
    for r in result {
        let r = r + 1;
        print!("{r} ");
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
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::stdin,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, RemAssign, Sub},
    str::FromStr,
    vec,
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
