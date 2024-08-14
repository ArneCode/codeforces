macro_rules! result {
    ($($arg:tt)*) => {
        #[cfg(feature = "arne-local")]
        {
            print!("Result is: ");
        }
        println!($($arg)*);
    };
}
fn trav(childs: &Vec<Vec<usize>>, i: usize, order: &mut Vec<usize>) {
    order.push(i);
    for child in &childs[i] {
        trav(childs, *child, order);
    }
}
fn get_size(childs: &Vec<Vec<usize>>, i: usize, sizes: &mut Vec<usize>) -> usize {
    let mut r = 1;
    for child in &childs[i] {
        r += get_size(childs, *child, sizes);
    }
    sizes[i] = r;
    r
}
fn cleanup(childs: &Vec<Vec<usize>>, i: usize, flips: &Vec<i64>, n_flips: i64, t: &mut Vec<i64>) {
    let n_flips = n_flips + flips[i];

    let r = [0, 1][((t[i] + n_flips) % 2) as usize];

    t[i] = r;

    for child in &childs[i] {
        cleanup(childs, *child, flips, n_flips, t);
    }
}
fn get_depth(childs: &Vec<Vec<usize>>, i: usize, depth: usize, depths: &mut Vec<usize>) {
    depths[i] = depth;
    for child in &childs[i] {
        get_depth(childs, *child, depth + 1, depths);
    }
}
fn solve(s: &mut String) {
    let [n] = read_vals_t(s, 0usize);

    let p: Vec<usize> = if n == 1 {
        read_line(s);
        vec![]
    } else {
        read_arr(s)
    };
    // println!("p: {p:?}");

    let mut parent = vec![0];

    let mut childs = vec![vec![]; n];

    for p in p {
        childs[p - 1].push(parent.len());
        parent.push(p - 1);
    }

    let mut order = vec![];
    trav(&childs, 0, &mut order);

    let mut sizes = vec![0; n];
    get_size(&childs, 0, &mut sizes);

    let mut depths = vec![0; n];
    get_depth(&childs, 0, 0, &mut depths);

    let mut n_to_order = vec![0; n];
    for (i, v) in order.iter().enumerate() {
        n_to_order[*v] = i;
    }

    let mut tree_arr: Vec<i64> = vec![];

    let mut t: Vec<i64> = read_arr(s);
    for n in &order {
        tree_arr.push(t[*n] as i64);
    }
    // println!("tree_arr: {tree_arr:?}");

    let mut tree = FenwickTree::from_iter(tree_arr.clone().into_iter());

    let [q] = read_vals_t(s, 0usize);

    let mut pows: Vec<usize> = vec![];
    let mut adjs = vec![];

    for _ in 0..q {
        let inp: Vec<String> = read_arr(s);
        let v: usize = inp[1].parse().unwrap();
        let v = v - 1;

        let pos = n_to_order[v];

        let mut n_above = 0;
        let mut lowest_above = 0;
        let mut depth_max = 0;
        for pow in &pows {
            let size = sizes[*pow];
            if pos <= n_to_order[*pow] {
                continue;
            }
            if pos >= n_to_order[*pow] + size {
                continue;
            }
            n_above += 1;

            let depth = depths[*pow];
            if depth > depth_max {
                lowest_above = *pow;
                depth_max = depth;
            }
        }
        let size = sizes[v];
        let mut adj_diff = 0;
        for (above_pos, adj_pos, diff) in &adjs {
            let above_pos = *above_pos;
            let adj_pos = *adj_pos;
            let diff = *diff;

            if lowest_above != above_pos {
                continue;
            }
            if adj_pos < pos {
                continue;
            }
            if adj_pos >= pos + size {
                continue;
            } 
            adj_diff = diff;
            break;
        }
        // println!("adj_diff: {adj_diff}, v: {}", v + 1);
        let n_on = tree.prefix_sum(pos + size, 0) - tree.prefix_sum(pos, 0) + adj_diff;
        let n_off = size as i64 - n_on;

        let n_on = [n_on, n_off][n_above % 2];
        let n_off = size as i64 - n_on;

        if &inp[0] == "pow" {
            let new = n_off;
            let old = n_on;
            // println!("old: {old}, n_on: {n_on}");
            let diff = new - old;
            // println!("before: {}", tree.prefix_sum(1, 0));
            // println!("{}, adding: {diff:?}", v + 1);

            tree.add_at(pos, diff);
            tree_arr[pos] += diff;

            // println!("diff: {diff}, tree_arr: {tree_arr:?}");

            if n_above % 2 == 1 {
                // println!("adj add, v: {}, diff: {}", v + 1, -diff*2);
                adjs.push((lowest_above, pos, -diff * 2));
            }

            // println!("after: {}", tree.prefix_sum(1, 0));

            pows.push(v);

            if pows.len() * pows.len() > n {
                // println!("cleanup");
                // cleanup
                let mut flips = vec![0; n];
                for pow in pows {
                    flips[pow] += 1;
                }
                pows = vec![];
                adjs = vec![];

                cleanup(&childs, 0, &flips, 0, &mut t);

                let mut tree_arr: Vec<i64> = vec![];
                for n in &order {
                    tree_arr.push(t[*n] as i64);
                }
                tree = FenwickTree::from_iter(tree_arr.clone().into_iter());
                // println!("cleanup complete!");
            }
        } else {
            result!("{n_on}");
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
extern crate alloc;
use alloc::vec::Vec;
use core::ops::{AddAssign, SubAssign};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct FenwickTree<T> {
    inner: Vec<T>,
}

impl<T> FromIterator<T> for FenwickTree<T>
where
    T: Copy + AddAssign,
{
    /// Creates a new fenwick tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use ftree::FenwickTree;
    ///
    /// let lengths: [usize; 5] = [1, 6, 3, 9, 2];
    /// // This is how lengths fenwick tree will look like internally
    /// let fenwick_tree: Vec<usize> = vec![1, 7, 3, 19, 2];
    /// // And this is how it can be constructed
    /// let initialized_fenwick_tree = FenwickTree::from_iter(lengths);
    /// ```
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut inner: Vec<T> = iter.into_iter().collect();
        let n = inner.len();

        for i in 0..n {
            let parent = i | (i + 1);
            if parent < n {
                let child = inner[i];
                inner[parent] += child;
            }
        }

        FenwickTree { inner }
    }
}

impl<const N: usize> From<[usize; N]> for FenwickTree<usize> {
    fn from(value: [usize; N]) -> Self {
        return FenwickTree::from_iter(value.into_iter());
    }
}

impl<T> FenwickTree<T> {
    /// Creates an empty(useless) fenwick tree.
    ///
    /// You might be looking for the method `from_iter` instead, which is the only
    /// way to create a useful fenwick tree.
    pub const fn new() -> Self {
        let inner = Vec::new();

        Self { inner }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FenwickTree<T> {
    /// Computes the prefix sum up until the desired index.
    ///
    /// The prefix sum up until the zeroth element is 0, since there is nothing before it.
    ///
    /// The prefix sum up until an index larger than the length is undefined, since every
    /// element after the length - 1 is undefined, therefore it will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use ftree::FenwickTree;
    ///
    /// let lengths = [1, 6, 3, 9, 2];
    /// let fenwick_array = FenwickTree::from_iter(lengths);
    ///
    /// let cases: Vec<(usize, usize)> =
    ///  vec![(0, 0), (1, 1), (2, 7), (3, 10), (4, 19), (5, 21)];
    ///
    /// cases
    ///   .into_iter()
    ///   .for_each(|(idx, expected_sum)| assert_eq!(fenwick_array.prefix_sum(idx, 0), expected_sum))
    /// ```
    pub fn prefix_sum(&self, index: usize, mut sum: T) -> T
    where
        T: Copy + AddAssign,
    {
        assert!(index < self.inner.len() + 1);

        let mut current_idx = index;

        while current_idx > 0 {
            sum += self.inner[current_idx - 1];
            current_idx &= current_idx - 1
        }

        sum
    }
    /// Increments a given index with a difference.
    ///
    /// # Examples
    ///
    /// ```
    /// use ftree::FenwickTree;
    ///
    /// let lengths = [1, 6, 3, 9, 2];
    /// let mut fenwick_array = FenwickTree::from_iter(lengths);
    ///
    /// let cases: Vec<(usize, usize)> = vec![(0, 0), (1, 2), (2, 8), (3, 11), (4, 20), (5, 22)];
    ///
    /// fenwick_array.add_at(0, 1);
    ///
    /// cases
    ///   .into_iter()
    ///   .for_each(|(idx, expected_sum)| assert_eq!(fenwick_array.prefix_sum(idx, 0), expected_sum))
    /// ```
    pub fn add_at(&mut self, index: usize, diff: T)
    where
        T: Copy + AddAssign,
    {
        let mut current_idx = index;

        while let Some(value) = self.inner.get_mut(current_idx) {
            *value += diff;
            current_idx |= current_idx + 1;
        }
    }
    /// Subtracts a difference from a given index.
    ///
    /// # Examples
    ///
    /// ```
    /// use ftree::FenwickTree;
    ///
    /// let lengths = [1, 6, 3, 9, 2];
    /// let mut fenwick_array = FenwickTree::from_iter(lengths);
    ///
    /// let cases: Vec<(usize, usize)> = vec![(0, 0), (1, 0), (2, 6), (3, 9), (4, 18), (5, 20)];
    ///
    /// fenwick_array.sub_at(0, 1);
    ///
    /// cases
    ///   .into_iter()
    ///   .for_each(|(idx, expected_sum)| assert_eq!(fenwick_array.prefix_sum(idx, 0), expected_sum))
    /// ```
    pub fn sub_at(&mut self, index: usize, diff: T)
    where
        T: Copy + SubAssign,
    {
        let mut current_idx = index;

        while let Some(value) = self.inner.get_mut(current_idx) {
            *value -= diff;
            current_idx |= current_idx + 1;
        }
    }
    /// Given a sum, finds the slot in which in which it would be "contained" within the original
    /// array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ftree::FenwickTree;
    ///
    /// let lengths = [1, 6, 3, 9, 2];
    /// let mut fenwick_array = FenwickTree::from_iter(lengths);
    ///
    /// let cases: Vec<(usize, usize)> = vec![(0, 0), (6, 1), (9, 2), (18, 3), (20, 4)];
    ///
    /// cases
    ///   .into_iter()
    ///   .for_each(|(prefix_sum, idx)| assert_eq!(fenwick_array.index_of(prefix_sum), idx))
    /// ```
    pub fn index_of(&self, mut prefix_sum: T) -> usize
    where
        T: Copy + Ord + SubAssign,
    {
        let mut index = 0;
        let mut probe = most_significant_bit(self.inner.len()) * 2;

        while probe > 0 {
            let lsb = least_significant_bit(probe);
            let half_lsb = lsb / 2;
            let other_half_lsb = lsb - half_lsb;

            if let Some(value) = self.inner.get(probe - 1) {
                if *value < prefix_sum {
                    index = probe;
                    prefix_sum -= *value;

                    probe += half_lsb;

                    if half_lsb > 0 {
                        continue;
                    }
                }
            }

            if lsb % 2 > 0 {
                break;
            }

            probe -= other_half_lsb;
        }

        index
    }
}

const fn least_significant_bit(n: usize) -> usize {
    n & n.wrapping_neg()
}

const fn most_significant_bit(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        1 << (usize::BITS - 1 - n.leading_zeros())
    }
}

use std::{
    fmt::Debug,
    io::stdin,
    ops::{Add, Div, Mul, MulAssign, Rem, RemAssign, Sub},
    str::FromStr,
    usize,
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
fn pow_mod<T>(mut b: T, mut e: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy,
{
    let mut r = T::from(1);
    while e > 0u64.into() {
        if e % 2u64.into() == 1u64.into() {
            r *= b;
            r %= mod_n;
        }
        b *= b;
        b %= mod_n;
        e >>= 1;
    }
    r
}
fn div_mod<T>(zähler: T, nenner: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>,
{
    (zähler * pow_mod(nenner, mod_n - 2u64.into(), mod_n)) % mod_n
}
fn fac_mod<T>(n: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    let mut r = 1u64.into();
    let mut i = 1u64.into();
    while i <= n {
        r *= i;
        r %= mod_n;
        i += 1u64.into();
    }
    r
}
fn binomial_koef_mod<T>(n: T, k: T, mod_n: T) -> T
where
    T: From<u64>
        + PartialOrd
        + Rem<Output = T>
        + RemAssign
        + Eq
        + MulAssign
        + std::ops::ShrAssign<i32>
        + Copy
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign,
{
    div_mod(
        fac_mod(n, mod_n),
        (fac_mod(n - k, mod_n) * fac_mod(k, mod_n)) % mod_n,
        mod_n,
    )
}
