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
    let [_, n] = read_vals(s);

    let trees: Vec<i128> = read_arr(s);

    let mut filled = HashSet::new();

    let mut placed = HashSet::new();

    let mut next = HashSet::new();

    for pos in trees {
        filled.insert(pos);
        next.insert(pos - 1);
        next.insert(pos + 1);
    }

    let mut nnext = HashSet::new();

    let mut dists = 0;

    let mut d = 1;
    while placed.len() < n {
        let placed_before = placed.len();
        for next in &next {
            if placed.len() >= n {
                break;
            }
            if filled.contains(next) {
                continue;
            }
            placed.insert(*next);
            filled.insert(*next);
            if !filled.contains(&(next + 1)) {
                nnext.insert(next + 1);
            }
            if !filled.contains(&(next - 1)) {
                nnext.insert(next - 1);
            }
        }
        dists += (placed.len() - placed_before)*d;
        d += 1;
        mem::swap(&mut nnext, &mut next);
        nnext.clear();
    }
    println!("{dists}");
    for p in placed {
        print!("{p} ");
    }
    println!()
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    io::stdin,
    mem,
    ops::{Add, Div, Sub},
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
