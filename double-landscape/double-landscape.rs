#[allow(unused_imports)]
use std::cmp::Ordering::{Equal, Greater, Less};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;

// double-landscape.rs --- Double Landscape
// author: Seong Yong-ju <sei40kr@gmail.com>

// tanakah's input macro
// cf https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

use std::cmp::Ordering;

/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [i64; n],
        ys: [i64; m],
    };

    let mut xs_sorted = xs;
    xs_sorted.sort();
    xs_sorted.dedup();
    let mut ys_sorted = ys;
    ys_sorted.sort();
    ys_sorted.dedup();

    let ans = if xs_sorted.len() == n && ys_sorted.len() == m {
        let mut ans = 1;

        for i in 1..((n * m) as i64 + 1) {
            let in_xs = {
                let j = xs_sorted.lower_bound(&i);
                j < n && xs_sorted[j] == i
            };
            let in_ys = {
                let j = ys_sorted.lower_bound(&i);
                j < m && ys_sorted[j] == i
            };

            let x_cands = if in_xs {
                1
            } else {
                n - xs_sorted.upper_bound(&i)
            } as i64;
            let y_cands = if in_ys {
                1
            } else {
                m - ys_sorted.upper_bound(&i)
            } as i64;

            if in_xs || in_ys {
                ans *= x_cands * y_cands;
            } else {
                ans *= x_cands * y_cands - ((n * m) as i64 - i);
            }

            ans %= 1_000_000_007;
        }

        ans
    } else {
        0
    };

    println!("{}", ans);
}
