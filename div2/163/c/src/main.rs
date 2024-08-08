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
    read_line(s);
    let arr1 = s.trim().chars().map(|c| c == '>').collect::<Vec<bool>>();
    read_line(s);
    let arr2 = s.trim().chars().map(|c| c == '>').collect::<Vec<bool>>();

    let mut visited = vec![vec![false; arr2.len()], vec![false; arr2.len()]];

    let arr = vec![arr1, arr2];

    let mut poses = vec![(0, 0)];
    let mut new_pos = vec![];

    while poses.len() > 0 {
        for (posy, posx) in &poses {
            let posy = *posy;
            let posx = *posx;
            if posy == 1 && posx == arr[0].len() - 1 {
                result!("yEs");
                return;
            }
            if posx < arr[0].len() - 2 && arr[posy][posx + 1] {
                let pos = (posy, posx + 2);
                if !visited[pos.0][pos.1] {
                    new_pos.push(pos);
                    visited[pos.0][pos.1] = true;
                }
            }
            if posy == 0 && arr[posy + 1][posx] && posx < arr[0].len() - 1 {
                let pos = (posy + 1, posx + 1);
                if !visited[pos.0][pos.1] {
                    new_pos.push(pos);
                    visited[pos.0][pos.1] = true;
                }
            }
            if posy == 1 && arr[posy - 1][posx] && posx < arr[0].len() - 1 {
                let pos = (posy - 1, posx + 1);
                if !visited[pos.0][pos.1] {
                    new_pos.push(pos);
                    visited[pos.0][pos.1] = true;
                }
            }
        }
        //new_pos = new_pos.into_iter().filter(|p| !visited.contains(p)).collect();
        mem::swap(&mut poses, &mut new_pos);
        new_pos.clear();
    }
    println!("nO");
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
    hash::Hash,
    io::stdin,
    mem,
    ops::{Add, Div, Sub},
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
