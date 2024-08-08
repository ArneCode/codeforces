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
    let mut rs: Vec<u64> = read_arr(s);
    let mut gs: Vec<u64> = read_arr(s);
    let mut bs: Vec<u64> = read_arr(s);

    rs.sort();
    gs.sort();
    bs.sort();

    let mut result = 0;

    let mut dp = vec![vec![vec![0; bs.len() + 1]; gs.len() + 1]; rs.len() + 1];

    let max_n = (rs.len() + gs.len() + bs.len());
    for n_tot in 0..=max_n {
        let min_r = 0.max(n_tot as i32 - gs.len() as i32 - bs.len() as i32) as usize;
        let max_r = n_tot.min(rs.len());
        //println!("n_tot: {n_tot}, min_r: {min_r}, max_r: {max_r}");
        for n_r in min_r..=max_r {
            let n_left = n_tot - n_r;
            let min_g = 0.max(n_left as i32 - bs.len() as i32) as usize;
            let max_g = n_left.min(gs.len());
            //println!("n_r: {n_r}, min_g: {min_g}, max_g: {max_g}");
            for n_g in min_g..=max_g {
                let n_b = n_left - n_g;
                //println!("n_g: {n_g}, n_b: {n_b}");
                let mut max = 0;
                if n_r > 0 && n_g > 0 {
                    let new_rect = rs[n_r - 1] * gs[n_g - 1];
                    let prev = dp[n_r - 1][n_g - 1][n_b];
                    max = max.max(prev + new_rect);
                }
                if n_r > 0 && n_b > 0 {
                    let new_rect = rs[n_r - 1] * bs[n_b - 1];
                    let prev = dp[n_r - 1][n_g][n_b - 1];
                    max = max.max(prev + new_rect);
                }
                if n_g > 0 && n_b > 0 {
                    let new_rect = gs[n_g - 1] * bs[n_b - 1];
                    let prev = dp[n_r][n_g - 1][n_b - 1];
                    max = max.max(prev + new_rect);
                }
                //println!("       r: {n_r}, g: {n_g}, b: {n_b}, max: {max}");
                dp[n_r][n_g][n_b] = max;
                result = result.max(max);
            }
        }
    }

    result!("{result}");
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
