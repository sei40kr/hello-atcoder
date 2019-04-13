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
    let x: i64;
    let y: i64;
    let z: i64;
    let k: i64;
    let As: Vec<i64>;
    let bs: Vec<i64>;
    let cs: Vec<i64>;

    input! {
        x: i64,
        y: i64,
        z: i64,
        k: i64,
        As: [i64; x as usize],
        bs: [i64; y as usize],
        cs: [i64; z as usize],
    };

    let mut abs = As
        .iter()
        .flat_map(|a| bs.iter().map(move |b| a + b))
        .collect::<Vec<_>>();
    abs.sort();

    let mut abcs = abs
        .iter()
        .rev()
        .take(k as usize)
        .flat_map(|ab| cs.iter().map(move |c| ab + c))
        .collect::<Vec<_>>();

    abcs.sort();

    for abc in abcs.into_iter().rev().take(k as usize) {
        println!("{}", abc);
    }
}
