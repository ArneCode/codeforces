macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn dfs(
    friends: &Vec<(u32, u64)>,
    solvers: &Vec<Vec<usize>>,
    curr: u32,
    i: usize,
    cache: &mut Vec<u64>,
) -> u64 {
    let cached_end_cost = cache[curr as usize];
    if cached_end_cost > 0 {
        return cached_end_cost;
    }
    let bit = 1 << i as u32;
    if curr & bit > 0 {
        if i == 0 {
            return 0;
        }
        return dfs(friends, solvers, curr, i - 1, cache);
    }
    let mut best = u64::MAX;
    for solver_i in &solvers[i] {
        let (bits, x) = friends[*solver_i];
        let new_curr = curr | bits;

        best = best.min(dfs(friends, solvers, new_curr, i, cache) + x);
    }
    cache[curr as usize] = best;
    return best;
}
fn solve(s: &mut String) {
    let [n, m, b] = read_vals_t(s, 0u64);

    let mut friends: Vec<(u32, u64, u64)> = vec![];
    let mut ks = vec![];
    for _ in 0..n {
        let [x, k, _] = read_vals_t(s, 0u64);
        let probs: Vec<u32> = read_arr(s);
        let mut bits = 0u32;

        ks.push(k);

        for prob in probs {
            let bit = 1 << (prob - 1);
            bits |= bit;
        }
        friends.push((bits, x, k));
    }
    ks.sort();
    let mut min = u64::MAX;
    'k_loop: for set_k in ks.into_iter().rev() {
        let mut new_friends = vec![];
        for (bits, x, k) in &friends {
            let bits = *bits;
            let x = *x;
            let k = *k;

            if k > set_k {
                continue;
            }
            new_friends.push((bits, x));
        }
        let friends = new_friends;
        let mut solvers = vec![vec![]; m as usize];
        for problem_i in 0..m {
            let bit = 1 << problem_i;
            for i in 0..friends.len() {
                let bits = friends[i].0;
                if bits & bit > 0 {
                    solvers[problem_i as usize].push(i);
                }
            }
            if solvers[problem_i as usize].is_empty() {
                break 'k_loop;
            }
            solvers[problem_i as usize].sort();
        }
        let mut cache = vec![0;2usize.pow(m as u32) + 1];
        let dfs_r = dfs(&friends, &solvers, 0, m as usize - 1, &mut cache);
        let r = dfs_r + set_k * b;
        min = min.min(r);
    }
    if min == u64::MAX {
        result!("-1");
    }else{
        result!("{min}");
    }
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    // for _ in 0..n_t {
    solve(&mut s);
    // }
}
use std::{
    collections::HashMap,
    fmt::Debug,
    io::stdin,
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
