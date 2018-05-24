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

// static-sushi.rs --- Static Sushi
// author: Seong Yong-ju <sei40kr@gmail.com>

#[allow(unused_macros)]
macro_rules! get {
  ($t:ty) => {
    {
      let mut line: String = String::new();
      stdin().read_line(&mut line).unwrap();
      line.trim().parse::<$t>().unwrap()
    }
  };
  ($($t:ty),*) => {
    {
      let mut line: String = String::new();
      stdin().read_line(&mut line).unwrap();
      let mut iter = line.split_whitespace();
      (
        $(iter.next().unwrap().parse::<$t>().unwrap(),)*
      )
    }
  };
  ($t:ty; $n:expr) => {
    (0..$n).map(|_|
                get!($t)
    ).collect::<Vec<_>>()
  };
  ($($t:ty),*; $n:expr) => {
    (0..$n).map(|_|
                get!($($t),*)
    ).collect::<Vec<_>>()
  };
  ($t:ty ;;) => {
    {
      let mut line: String = String::new();
      stdin().read_line(&mut line).unwrap();
      line.split_whitespace()
        .map(|t| t.parse::<$t>().unwrap())
        .collect::<Vec<_>>()
    }
  };
  ($t:ty ;; $n:expr) => {
    (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
  };
}

#[allow(unused_macros)]
macro_rules! debug {
  ($($a:expr),*) => {
    eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
  }
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("Failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("Failed to parse token")
}

#[allow(dead_code)]
fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
    let out = stdout();
    let writer = BufWriter::new(out.lock());
    f(writer);
}

fn main() {
    let (n, c) = get!(usize, i64);
    let mut sushis = get!(i64, i64; n);

    // clockwise
    let cw_ds_max = {
        let mut v_sum = 0;
        let (mut c_max1, mut c_max2) = (0, 0);

        sushis
            .iter()
            .map(|&(x, v)| {
                v_sum += v;
                c_max1 = max(c_max1, v_sum - x);
                c_max2 = max(c_max2, v_sum - 2 * x);
                (c_max1, c_max2)
            })
            .collect::<Vec<_>>()
    };

    // anti-clockwise
    sushis.reverse();
    let mut acw_ds_max = {
        let mut v_sum = 0;
        let (mut c_max1, mut c_max2) = (0, 0);

        sushis
            .iter()
            .map(|&(x, v)| {
                v_sum += v;
                c_max1 = max(c_max1, v_sum - (c - x));
                c_max2 = max(c_max2, v_sum - 2 * (c - x));
                (c_max1, c_max2)
            })
            .collect::<Vec<_>>()
    };
    acw_ds_max.reverse();

    let ans = max(
        cw_ds_max
            .into_iter()
            .enumerate()
            .map(|(i, (cw_c_max1, cw_c_max2))| {
                if i != n - 1 {
                    let (acw_c_max1, acw_c_max2) = acw_ds_max[i + 1];
                    max(cw_c_max1 + acw_c_max2, acw_c_max1 + cw_c_max2)
                } else {
                    cw_c_max1
                }
            })
            .max()
            .unwrap(),
        acw_ds_max
            .into_iter()
            .map(|(c_max1, _)| c_max1)
            .max()
            .unwrap(),
    );

    with_bufwriter(|mut out| writeln!(out, "{}", ans).unwrap());
}
