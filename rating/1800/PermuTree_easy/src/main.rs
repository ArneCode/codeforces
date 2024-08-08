macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn get_childs(verts: &Vec<Vec<usize>>, result: &mut Vec<u64>, i: usize) {
    let mut n_childs = 0;
    for child in &verts[i] {
        get_childs(verts, result, *child);
        n_childs += result[*child];
    }
    result[i] = n_childs + 1;
}
fn max_f(verts: &Vec<Vec<usize>>, n_childs: &Vec<u64>, i: usize) -> u64 {
    let mut result = 0;
    for child in &verts[i] {
        result += max_f(verts, n_childs, *child);
    }
    let max = n_childs[i] - 1;
    let mut reachable = HashSet::new();
    let mut amnts = HashMap::new();
    for child in &verts[i] {
        let amnt = n_childs[*child];
        if amnt*2 >= max {
            result += amnt*(max - amnt);
            return result;
        }
        if let Some(n) = amnts.get_mut(&amnt) {
            *n += 1;
        } else {
            amnts.insert(amnt, 1u64);
        }
    }
    reachable.insert(0);
    //println!("amnts.: {amnts:?}");
    let mut min_dist = u64::MAX;
    'outer: for (amnt, n) in amnts {
        let mut new_elts = vec![];
        for v in &reachable {
            for n in 1..=n {
                let new_v = amnt * n + *v;
                if reachable.contains(&new_v) || (new_v*2 > max && (max/2 - new_v)>min_dist) {
                    break;
                } else {
                    new_elts.push(new_v);
                }
            }
        }
        for elt in new_elts {
            let dist = (max/2).abs_diff(elt);
            min_dist = min_dist.min(dist);
            reachable.insert(elt);
            if elt == max/2 {
                break 'outer;
            }
        }
    }
    // for child in &verts[i] {
    //     let mut new_elts = vec![];
    //     for v in &reachable {
    //         let new_v = n_childs[*child] + *v;
    //         if new_v < max {
    //             new_elts.push(new_v);
    //         }
    //     }
    //     for elt in new_elts {
    //         reachable.insert(elt);
    //     }
    //     reachable.insert(n_childs[*child]);
    // }
    let mut nearest = 0;
    for r in reachable {
        if r * 2 <= max {
            nearest = nearest.max(r);
        }
    }
    result += nearest * (max - nearest);
    result
}
fn solve(s: &mut String) {
    let [n] = read_vals_t(s, 0usize);
    let a: Vec<usize> = read_arr(s);
    let a: Vec<usize> = a.into_iter().map(|v| v - 1).collect();
    // assert!(a.len() == n - 1);

    let mut verts = vec![vec![]; n];

    for (i, v) in a.iter().enumerate() {
        verts[*v].push(i + 1);
    }
    let mut n_childs = vec![0; n];
    get_childs(&verts, &mut n_childs, 0);
    let n_childs = n_childs;
    let result = max_f(&verts, &n_childs, 0);
    result!("{result}");
}
fn main() {
    let mut s = String::new();

    // let [n_t] = read_vals(&mut s);

    solve(&mut s);
}
use std::{
    char,
    collections::{HashMap, HashSet},
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
