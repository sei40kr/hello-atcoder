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

// ringos-favorite-numbers.rs --- Ringo's Favorite Numbers
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
    println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
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
    let (d, n) = get!(u32, u32);

    let ans = 100u32.pow(d) * (if n == 100 { n + 1 } else { n });

    with_bufwriter(|mut out| writeln!(out, "{}", ans).unwrap());
}
