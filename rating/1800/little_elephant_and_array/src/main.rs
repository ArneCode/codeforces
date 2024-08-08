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
    let [_, n_q] = read_vals(s);
    let a: Vec<u128> = read_arr(s);
    let mut amnts = vec![0;a.len() + 1];
    for v in &a {
        if *v <= a.len() as u128 {
            amnts[*v as usize] += 1; 
        }
    }
    let mut possible_map = HashMap::new();
    let mut possible = vec![];

    for (i, amnt) in amnts.iter().enumerate().skip(1) {
        if *amnt >= i {
            possible_map.insert(i as u128, possible_map.len());
            possible.push(i);
        }
    }
    let mut sums = vec![0;possible_map.len()];
    let mut prefixes:Vec<Vec<u32>>  = vec![Vec::with_capacity(a.len());possible_map.len()];
    for prefix in &mut prefixes {
        prefix.push(0);
    }
    for v in &a {
        if let Some(pos) = possible_map.get(v) {
            sums[*pos] += 1;
        }
        for i in 0..sums.len() {
            prefixes[i].push(sums[i]);
        }
    }
    //println!("prefixes: {prefixes:?}");
    for _ in 0..n_q {
        let [l, r] = read_vals_t(s, 0usize);

        //println!("l: {l}, r: {r}");
        let mut result = 0;
        for (prefix, i) in prefixes.iter().zip(possible.iter()) {
            let amnt = prefix[r] - prefix[l - 1];
            if amnt == *i as u32 {
                result += 1;
            }
            //println!("i: {i}, amnt: {amnt}");
        }
        result!("{result}");
    }
}
fn main() {
    let mut s = String::new();

    solve(&mut s);
}
use std::{
    collections::{HashMap, HashSet}, fmt::Debug, io::stdin, ops::{Add, Div, Sub}, str::FromStr
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
