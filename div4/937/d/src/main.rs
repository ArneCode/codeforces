macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn get_binary_dec(n: u64) -> Vec<u64> {
    let mut c = 1;
    let mut result = vec![];
    let mut r_n = 1;
    while r_n < n {
        result.push(r_n);
        c += 1;
        r_n = 0;
        //let mut c_c = c;
        for i in 0u32..32 {
            //println!("i: {i}");
            let bit = 1 << i;
            if (c & bit) > 0 {
                r_n += 10u64.pow(i);
            }
            // r_n += c_c % 2;
            // c_c >>= 1;
            // r_n *= 10;
        }
        //r_n /= 10;
    }
    //println!("reslt: {:?}", &result[0..10]);
    return result;
}
fn r(n: u64, bs: &Vec<u64>, set: &HashSet<u64>) -> bool {
    //println!("n: {n}");
    if set.contains(&n) {
        return true;
    }
    for b in bs {
        if *b == 1 {
            continue;
        }
        if n % *b == 0 {
            if r(n/(*b), bs, set){
                return true;
            }
        }
        // if b*b > n {
        //     break;
        // }
    }
    return false;
}
fn solve(s: &mut String, b: &Vec<u64>, set: &HashSet<u64>) {
    let [n] = read_vals_t(s, 0u64);
    if r(n, b, &set) {
        println!("yEs");
    }else{
        println!("nO");
    }
}
fn main() {
    let mut s = String::new();

    let [n_t] = read_vals(&mut s);
    let b = get_binary_dec(10u64.pow(6) + 3);
    let mut set = HashSet::new();
    for b in &b {
        set.insert(*b);
    }
    for _ in 0..n_t {
        solve(&mut s, &b, &set);
    }
}
use std::{
    collections::HashSet, fmt::Debug, io::stdin, ops::{Add, Div, Sub}, str::FromStr
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
