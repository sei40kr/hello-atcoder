#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;

// alternating-path.rs --- Alternating Path
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

fn f(
    h: usize,
    w: usize,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    expect: char,
) -> (u64, u64) {
    if !visited[y][x] && map[y][x] == expect {
        visited[y][x] = true;

        match [
            (x as i32 - 1, y as i32),
            (x as i32 + 1, y as i32),
            (x as i32, y as i32 - 1),
            (x as i32, y as i32 + 1),
        ]
        .into_iter()
        .filter(|&&(x, y)| 0 <= x && x < w as i32 && 0 <= y && y < h as i32)
        .map(|&(x, y)| (x as usize, y as usize))
        .map(|(x, y)| {
            f(
                h,
                w,
                map,
                visited,
                x,
                y,
                if expect == '#' { '.' } else { '#' },
            )
        })
        .fold((0, 0), |(b_sum, w_sum), (b, w)| (b_sum + b, w_sum + w))
        {
            (b, w) => {
                if map[y][x] == '#' {
                    (b + 1, w)
                } else {
                    (b, w + 1)
                }
            }
        }
    } else {
        (0, 0)
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        map: [chars; h],
    };

    let mut visited = vec![vec![false; w]; h];
    let mut ans = 0;

    for y in 0..h {
        for x in 0..w {
            let (b, w) = f(h, w, &map, &mut visited, x, y, map[y][x]);
            ans += b * w;
        }
    }

    println!("{}", ans);
}
