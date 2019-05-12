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

// author: Seong Yong-ju <sei40kr@gmail.com>

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
    let n: i64;

    input! {
        n: i64,
    };

    let sqrt_n = (n as f64).sqrt() as i64;

    let mut divisors = (1..sqrt_n + 1)
        .filter(|k| n % k == 0)
        .flat_map(|k| vec![k, n / k])
        .collect::<Vec<_>>();
    divisors.sort();
    divisors.dedup();

    let ans = divisors
        .into_iter()
        .map(|d| d - 1)
        .filter(|&k| k != 0 && n / k == n % k)
        .sum::<i64>();

    println!("{}", ans);
}
