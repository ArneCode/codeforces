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
    let [n] = read_vals_t(s, 0usize);
    let mut ls = HashMap::new();
    let mut rs = HashMap::new();

    let mut known_pts = HashSet::new();
    let mut pts_heap = BinaryHeap::new();

    let mut segments = vec![];
    for _ in 0..n {
        let [l, r] = read_vals_t(s, 0u64);
        if let Some(amnt) = ls.get_mut(&l) {
            *amnt += 1;
        } else {
            ls.insert(l, 1);
        }
        if let Some(amnt) = rs.get_mut(&r) {
            *amnt += 1;
        } else {
            rs.insert(r, 1);
        }
        if !known_pts.contains(&l) {
            pts_heap.push(Reverse(l));
            known_pts.insert(l);
        }
        if !known_pts.contains(&r) {
            pts_heap.push(Reverse(r));
            known_pts.insert(r);
        }
        segments.push((l, r));
    }

    let mut pts = Vec::with_capacity(pts_heap.len()); //pts.into_iter().map(|elt| elt.0).collect::<Vec<_>>();

    while pts_heap.len() > 0 {
        pts.push(pts_heap.pop().unwrap().0);
    }
    let mut ends_before = Vec::with_capacity(pts.len() + 1);

    let mut n_ended = 0;
    for pt in &pts {
        ends_before.push(n_ended);
        if let Some(amnt) = rs.get(pt) {
            n_ended += amnt;
        }
    }

    let mut starts_after = vec![0; pts.len()];
    let mut starts = 0;
    for i in (0..pts.len()).rev() {
        starts_after[i] = starts;
        if let Some(amnt) = ls.get(&pts[i]) {
            starts += *amnt;
        }
    }

    //println!("ends_before: {ends_before:?}");
    //println!("starts_after: {starts_after:?}");

    let mut reverse = HashMap::with_capacity(pts.len());

    for (i, pt) in pts.iter().enumerate() {
        reverse.insert(*pt, i);
    }
    //println!("reverse: {reverse:?}");
    //println!("pts: {pts:?}");
    let mut min = u64::MAX;
    for (l, r) in segments {
        let l_i = *reverse.get(&l).unwrap();
        let r_i = *reverse.get(&r).unwrap();

        let r = ends_before[l_i] + starts_after[r_i];
        min = min.min(r);
    }
    result!("{min}");
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
    collections::{BinaryHeap, HashMap, HashSet},
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
