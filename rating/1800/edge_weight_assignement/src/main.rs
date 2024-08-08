macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn dfs(verts: &Vec<Vec<usize>>, i: usize, visited: &mut Vec<bool>) -> (u32, u32, u32) {
    visited[i] = true;
    let mut min = 1;
    let mut max = 0;
    let mut had_1 = false;
    let mut depth = 0;
    let mut even_depth = false;
    let mut odd_depth = false;

    let mut no_childs = true;
    for child in &verts[i] {
        if visited[*child] {
            continue;
        }
        no_childs = false;

        let (child_depth, child_min, child_max) = dfs(verts, *child, visited);
        depth = depth.max(child_depth);
        min = min.max(child_min);
        if child_depth == 0 {
            if !had_1 {
                had_1 = true;
                max += 1;
            }
        } else {
            max += child_max + 1;
        }
        if child_depth % 2 == 0 {
            even_depth = true;
        } else {
            odd_depth = true;
        }
    }
    if even_depth && odd_depth {
        min = 3;
    }
    if !no_childs {
        depth += 1;
    }
    if i == 0 && verts[0].len() == 1 {
        if depth == 2 {
            max = 1;
        }
        if depth % 2 == 1 {
            min = min.max(3);
        }
    }
    //println!("{}: depth: {}, min: {}, max: {}", i + 1, depth, min, max);
    (depth, min, max)
}
fn solve(s: &mut String) {
    let [n] = read_vals_t(s, 0u64);
    let mut verts = vec![vec![]; n as usize];
    for _ in 1..n {
        let [f, t] = read_vals_t(s, 0usize);
        verts[f - 1].push(t - 1);
        verts[t - 1].push(f - 1);
    }
    let mut visited = vec![false; n as usize];
    for i in 0..verts.len() {
        if verts[i].len() > 1 {
            let (_, min, max) = dfs(&verts, i, &mut visited);
            result!("{min} {max}");
            return;
        }
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
