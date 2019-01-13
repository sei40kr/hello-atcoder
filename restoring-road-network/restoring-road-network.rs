#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;

// restoring-road-network.rs --- Restoring Road Network
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

fn main() {
    input! {
        n: usize,
        edges: [[i64; n]; n],
    };

    let mut dp = edges;
    let mut needed = vec![vec![true; n]; n];
    let mut exists = true;

    for k in 0..n {
        for i in (0..n).filter(|&i| i != k) {
            for j in (i..n).filter(|&j| j != k) {
                let t = dp[i][k] + dp[k][j];

                match dp[i][j].cmp(&t) {
                    Ordering::Equal => {
                        needed[i][j] = false;
                    }
                    Ordering::Greater => {
                        dp[i][j] = t;
                        needed[i][j] = false;
                        exists = false;
                    }
                    _ => {}
                }
                if t <= dp[i][j] {
                    dp[i][j] = t;
                    needed[i][j] = false;
                }
            }
        }
    }

    let ans = if exists {
        (0..n)
            .map(|i| {
                (i..n)
                    .filter(|&j| needed[i][j])
                    .map(|j| dp[i][j])
                    .sum::<i64>()
            })
            .sum::<i64>()
    } else {
        -1
    };
    println!("{}", ans);
}
