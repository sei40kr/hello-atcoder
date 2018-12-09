#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Read, StdoutLock, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(unused_imports)]
use std::str::FromStr;

// 755.rs --- 755
// author: Seong Yong-ju <sei40kr@gmail.com>

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
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

#[allow(dead_code)]
fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
    let out = stdout();
    let writer = BufWriter::new(out.lock());
    f(writer);
}

fn num(e: &[i32], m: i32) -> usize {
    return e.iter().filter(|x| m - 1 <= **x).count();
}

fn main() {
    input! {
      n:i32
    }
    let mut e = vec![0i32; n as usize];

    for mut i in 2..n + 1 {
        for j in 2..i + 1 {
            while i % j == 0 {
                e[j as usize] += 1;
                i /= j;
            }
        }
    }

    let ans = num(&e, 75)
        + num(&e, 25) * (num(&e, 3) - 1)
        + num(&e, 15) * (num(&e, 5) - 1)
        + num(&e, 5) * (num(&e, 5) - 1) * (num(&e, 3) - 2) / 2;

    with_bufwriter(|mut out| {
        writeln!(out, "{}", ans);
    });
}
