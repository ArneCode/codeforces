macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn calc_good(
    p: &Vec<usize>,
    childs: &Vec<Vec<usize>>,
    parents: &Vec<usize>,
    i: usize,
    goods: &mut Vec<bool>,
    tot_child: &Vec<usize>,
) -> bool {
    let mut good = true;
    // for child in &childs[i] {
    //     calc_good(p, childs, *child, visited, goods);
    //     good = good && goods[*child];
    // }
    let mut j = 1;
    let mut n_child = 0;
    while n_child < childs[p[i]].len() && i + j < p.len() {
        //good = good && goods[i + j]; //&& calc_good(p, childs, i + j, visited, goods, tot_child)
        if p[i + j] == 0 || parents[p[i + j] - 1] != p[i] {
            good = false;
            break;
        }
        j += tot_child[p[i + j]];
        n_child += 1;
    }
    if n_child < childs[p[i]].len() {
        good = false;
    }
    goods[i] = good;
    good
}
fn calc_tot_child(childs: &Vec<Vec<usize>>, i: usize, tot_child: &mut Vec<usize>) -> usize {
    //println!("tot_child, {i}");
    let mut r: usize = 1;
    for child in &childs[i] {
        r += calc_tot_child(childs, *child, tot_child);
    }
    tot_child[i] = r;
    r
}
fn solve(s: &mut String) {
    let [n, q] = read_vals_t(s, 0usize);

    let mut parents: Vec<usize> = read_arr(s);
    for p in parents.iter_mut() {
        *p -= 1;
    }
    let mut childs = vec![vec![]; n];
    for i in 0..parents.len() {
        childs[parents[i]].push(i + 1);
    }

    let mut p: Vec<usize> = read_arr(s);

    for p in p.iter_mut() {
        *p -= 1;
    }

    let mut good = vec![false; n];

    let mut tot_child = vec![0; n];

    calc_tot_child(&childs, 0, &mut tot_child);

    for i in 0..n {
        calc_good(&p, &childs, &parents, i, &mut good, &tot_child);
    }

    let mut n_bad = good.iter().filter(|v| !*v).count();

    let mut idx = vec![0; n];
    for i in 0..p.len() {
        idx[p[i]] = i;
    }
    for _ in 0..q {
        let [x, y] = read_vals_t(s, 0usize);
        let x = x - 1;
        let y = y - 1;

        let px = p[x];
        let py = p[y];

        p[x] = py;
        p[y] = px;

        idx[py] = x;
        idx[px] = y;

        let min = x.min(y);
        let max = y.max(x);

        let prev = good[min];
        let new = calc_good(&p, &childs, &parents, min, &mut good, &tot_child);

        if prev && !new {
            n_bad += 1;
        }
        if !prev && new {
            n_bad -= 1;
        }

        if p[min] != 0 && new {
            let parent = idx[parents[p[min] - 1]];

            let prev = good[parent];
            let new = calc_good(&p, &childs, &parents, parent, &mut good, &tot_child);

            if prev && !new {
                n_bad += 1;
            }
            if !prev && new {
                n_bad -= 1;
            }
        }

        let prev = good[max];
        let new = calc_good(&p, &childs, &parents, max, &mut good, &tot_child);

        if prev && !new {
            n_bad += 1;
        }
        if !prev && new {
            n_bad -= 1;
        }

        if p[max] != 0 && new {
            let parent = idx[parents[p[max] - 1]];

            let prev = good[parent];
            let new = calc_good(&p, &childs, &parents, parent, &mut good, &tot_child);

            if prev && !new {
                n_bad += 1;
            }
            if !prev && new {
                n_bad -= 1;
            }
        }

        if n_bad == 0 {
            result!("yEs");
        } else {
            result!("nO");
        }
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
    mem::transmute,
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
