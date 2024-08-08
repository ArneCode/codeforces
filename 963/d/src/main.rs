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
    let [n, k] = read_vals_t(s, 0usize);

    let a: Vec<u64> = read_arr(s);

    let mut upper: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    let mut lower: BinaryHeap<u64> = BinaryHeap::new();

    for i in 0..k {
        lower.push(a[i]);
        if lower.len() > upper.len() + 1 {
            upper.push(Reverse(lower.pop().unwrap()));
        }
    }

    let mut meds = vec![];

    meds.push(*lower.peek().unwrap());

    let mut rl: HashMap<u64, u64> = HashMap::new();
    let mut ru: HashMap<u64, u64> = HashMap::new();

    for i in 0..(n - k) {
        let curr_med = *meds.last().unwrap();
        if a[i] > curr_med {
            if let Some(amnt) = ru.get_mut(&a[i]) {
                *amnt += 1;
            } else {
                ru.insert(a[i], 1);
            }
        } else {
            if let Some(amnt) = rl.get_mut(&a[i]) {
                *amnt += 1;
            } else {
                rl.insert(a[i], 1);
            }
        }

        lower.push(a[i + k]);

        let mut done = false;

        while lower.len() - rl.len() > upper.len() - ru.len() + 1 {
            let elt = lower.pop().unwrap();
            if let Some(amnt) = rl.get_mut(&elt) {
                if *amnt > 0 {
                    *amnt -= 1;
                    continue;
                }
            }
            upper.push(Reverse(elt));
            done = true;
        }
        while lower.len() - rl.len() < upper.len() - ru.len() {
            let elt = upper.pop().unwrap().0;
            if let Some(amnt) = ru.get_mut(&elt) {
                if *amnt > 0 {
                    *amnt -= 1;
                    continue;
                }
            }
            lower.push(elt);
            done = true;
        }

        if !done {
            loop {
                let elt = *lower.peek().unwrap();
                if let Some(amnt) = rl.get_mut(&elt) {
                    if *amnt > 0 {
                        *amnt -= 1;
                        continue;
                    }
                }
                break;
            }
        }
        meds.push(*lower.peek().unwrap());
    }
    println!("meds: {meds:?}");
    // let mut sorted = meds.iter().copied().enumerate().collect::<Vec<_>>();
    // sorted.sort_by_key(|(_, k)| *k);

    let remove_below = binary_edge(
        |remove_below| -> bool {
            let mut amnt = n;
            let mut i = 0;
            while i < meds.len() && amnt > k {
                if meds[i] < remove_below {
                    amnt -= k;
                    i += k;
                    continue;
                }
                i += 1;
            }
            amnt <= k
        },
        *meds.iter().min().unwrap(),
        *meds.iter().max().unwrap() + 1,
    );
    let mut f = vec![];
    let mut i = 0;
    while i < a.len() {
        if i < meds.len() && meds[i] < remove_below {
            i += k;
            continue;
        }
        f.push(a[i]);
        i += 1;
    }

    f.sort();
    println!("f: {f:?}");
    let r = f[((f.len() + 1) / 2) - 1];
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
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
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
