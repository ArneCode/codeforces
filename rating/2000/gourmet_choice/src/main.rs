macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Ord {
    Greater,
    Eq,
    Less,
}
fn solve(s: &mut String) {
    let [n, m] = read_vals_t(s, 0usize);
    let mut ns_greater = Vec::with_capacity(n);
    let mut vals = Vec::with_capacity(n);

    for i in 0..n {
        read_line(s);
        let s = s.trim();
        let mut n_greater = 0;
        let mut ord = vec![];
        for c in s.chars() {
            let o = match c {
                '>' => {
                    n_greater += 1;
                    Ord::Greater
                }
                '=' => {
                    n_greater += 1;
                    Ord::Eq
                }
                '<' => Ord::Less,
                _ => panic!(),
            };
            ord.push(o);
        }
        ns_greater.push((n_greater, i));
        vals.push(ord);
    }
    ns_greater.sort_by_key(|(n_less, _)| *n_less);

    let mut orderings_a = vec![];
    let mut orderings_b = vec![];
    let mut i = 0;

    let mut prev_a = vec![Ord::Less; m];
    let mut prev = &prev_a;
    while i < ns_greater.len() {
        let (n_greater, ord_i) = ns_greater[i];
        let new = &vals[ord_i];

        let mut prev_bs = vec![];
        let mut new_bs = vec![];
        let mut new_as = vec![];

        for (i, (prev, new)) in prev.iter().zip(new.iter()).enumerate() {
            if prev == &Ord::Less {
                if new == &Ord::Eq {
                    new_bs.push(i);
                } else if new == &Ord::Greater {
                    prev_bs.push(i)
                }
            }
            if (prev == &Ord::Eq && new == &Ord::Less)
                || (prev == &Ord::Greater && (new == &Ord::Eq || new == &Ord::Less))
            {
                println!("No");
                return;
            }
        }
        new_as.push(ord_i);
        i += 1;
        while i < ns_greater.len() && ns_greater[i].0 == n_greater {
            new_as.push(ns_greater[i].1);
            i += 1;
        }
        if prev_bs.len() > 0 {
            orderings_a.push(vec![]);
            orderings_b.push(prev_bs);
        }
        orderings_a.push(new_as);
        orderings_b.push(new_bs);
        prev = new;
    }
    // let mut new_as = vec![];
    let mut new_bs = vec![];
    for (i, elt) in prev.iter().enumerate() {
        if elt == &Ord::Less {
            new_bs.push(i)
        }
    }
    if !new_bs.is_empty() {
        orderings_a.push(vec![]);
        orderings_b.push(new_bs);
    }
    //println!("orderings_a: {orderings_a:?}, orderings_b: {orderings_b:?}");

    let mut nums_a = vec![None; n];
    let mut nums_b = vec![None; m];

    for num in 0..orderings_a.len() {
        let a_s = &orderings_a[num];
        let b_s = &orderings_b[num];
        if !a_s.is_empty() {
            let mut prev = a_s[0];
            for elt in a_s.iter().skip(1) {
                for (prev, curr) in vals[prev].iter().zip(vals[*elt].iter()) {
                    if prev != curr {
                        println!("NO");
                        return;
                    }
                }
                prev = *elt;
            }
            for b in b_s {
                if vals[prev][*b] != Ord::Eq {
                    println!("NO");
                    return;
                }
            }
        }
        if !b_s.is_empty() {
            let mut prev = b_s[0];
            for elt in b_s.iter().skip(1) {
                for row in &vals {
                    let prev = &row[prev];
                    let curr = &row[*elt];
                    if prev != curr {
                        println!("No");
                        return;
                    }
                }
                prev = *elt;
            }
            //     for (prev, curr) in vals[prev].iter().zip(vals[*elt].iter()) {
            //         if prev != curr {
            //             println!("No");
            //             return;
            //         }
            //     }
            //     prev = *elt;

            for a in a_s {
                if vals[*a][prev] != Ord::Eq {
                    println!("No");
                    return;
                }
            }
        }
        for a in a_s {
            if nums_a[*a].is_some() {
                println!("nO");
                return;
            }
            nums_a[*a] = Some(num);
        }
        for b in b_s {
            if nums_b[*b].is_some() {
                println!("nO");
                return;
            }
            nums_b[*b] = Some(num);
        }
    }
    for num in &nums_a {
        if num.is_none() {
            println!("nO");
            return;
        }
    }
    for num in &nums_b {
        if num.is_none() {
            println!("nO");
            return;
        }
    }
    for y in 0..n {
        for x in 0..m {
            let ord = &vals[y][x];
            let good = match *ord {
                Ord::Greater => nums_a[y].unwrap()>nums_b[x].unwrap(),
                Ord::Eq => nums_a[y].unwrap() == nums_b[x].unwrap(),
                Ord::Less => nums_a[y].unwrap() < nums_b[x].unwrap(),
            };
            if !good{
                println!("No");
                return;
            }
        }
    }
    println!("yes");
    for num in nums_a {
        print!("{} ", num.unwrap() + 1);
    }
    println!();
    for num in nums_b {
        print!("{} ", num.unwrap() + 1);
    }
    println!();
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
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
