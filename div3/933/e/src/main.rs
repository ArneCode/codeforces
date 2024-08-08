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
    let [n, _, k, d] = read_vals(s);

    let mut results: Vec<u128> = vec![];
    for _ in 0u128..n {
        let line: Vec<u128> = read_arr(s);
        let mut heap: BinaryHeap<Reverse<u128>> = BinaryHeap::new();
        let mut map: HashMap<u128, u32> = HashMap::new();
        let mut queue: VecDeque<u128> = VecDeque::new();
        heap.push(Reverse(1));
        map.insert(1, 1);
        queue.push_back(1);

        for i in 1..(line.len() - 1) {
            let min = loop {
                let min = heap.peek().unwrap().0;
                if map.get(&min).unwrap() > &0 {
                    break min;
                }
                heap.pop();
            };
            let new_v = min + line[i] + 1;
            heap.push(Reverse(new_v));
            queue.push_back(new_v);
            if let Some(amnt) = map.get_mut(&new_v) {
                *amnt += 1;
            } else {
                map.insert(new_v, 1);
            }

            if heap.len() > d as usize + 1 {
                let out = queue.pop_front().unwrap();
                *map.get_mut(&out).unwrap() -= 1;
            }
        }
        let min = loop {
            let min = heap.peek().unwrap().0;
            if map.get(&min).unwrap() > &0 {
                break min;
            }
            heap.pop();
        };
        results.push(min + 1);
        //println!("queue: {queue:?}");
    }
    results.sort();
    //println!("results: {results:?}");
    let r: u128 = results[0..(k as usize)].iter().sum();
    result!("{r}");
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
    collections::{BinaryHeap, HashMap, VecDeque},
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
