macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn r(
    verts: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    edge_map: &HashMap<(usize, usize), u64>,
    i: usize,
) -> Option<(u64, Vec<u64>)> {
    //println!("AAAAA");
    visited[i] = true;
    let mut cuts = vec![];
    let mut size = 1;
    for child in &verts[i] {
        if visited[*child] {
            continue;
        }
        let (child_size, child_cuts) = r(verts, visited, edge_map, *child)?;
        cuts.extend(child_cuts);
        if child_size == 3 {
            let edge = edge_map.get(&(i, *child)).unwrap();
            cuts.push(*edge);
        } else if child_size > 3 {
            return None;
        } else {
            size += child_size;
        }
    }
    // if cuts.len() > 0 {
    //     println!("test");
    // }
    if i == 0 && size != 3 {
        return None;
    }
    //println!("i: {i}, size: {size}");
    Some((size, cuts))
}
fn solve(s: &mut String) {
    let [n] = read_vals_t(s, 0u64);
    let mut verts = vec![vec![]; n as usize];

    let mut edge_map = HashMap::new();
    for i in 1..n {
        let [u, v] = read_vals_t(s, 0usize);
        verts[u - 1].push(v - 1);
        verts[v - 1].push(u - 1);

        // let len = edge_map.len() + 1;
        edge_map.insert((u - 1, v - 1), i);
        edge_map.insert((v - 1, u - 1), i);
    }
    let mut visited = vec![false;n as usize];
    if let Some((_, cuts)) = r(&verts, &mut visited, &edge_map, 0){
        let out = &mut BufWriter::new(stdout());
        writeln!(out, "{}", cuts.len()).ok();
        for cut in cuts {
            write!(out, "{cut} ").ok();
        }
        writeln!(out).ok();
    }else{
        println!("-1");
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
    collections::HashMap,
    fmt::Debug,
    io::{stdin, stdout, BufWriter, Write},
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
