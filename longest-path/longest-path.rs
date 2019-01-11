// longest-path.rs --- Longest Path
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

fn f(dp: &mut Vec<u32>, paths: &Vec<Vec<usize>>, i: usize) -> u32 {
    if dp[i] != 0 {
        return dp[i];
    }

    dp[i] = paths[i]
        .iter()
        .map(|&j| f(dp, paths, j))
        .max()
        .map_or(0, |l| l + 1);
    return dp[i];
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize1, usize1); m],
    };

    let mut paths = vec![vec![]; n];
    let mut refs = vec![vec![]; n];
    for (x, y) in edges.into_iter() {
        paths[x].push(y);
        refs[y].push(x);
    }
    let mut dp = vec![0; n];

    let ans = refs
        .into_iter()
        .enumerate()
        .filter(|&(_, ref from)| from.is_empty())
        .map(|(i, _)| f(&mut dp, &paths, i))
        .max()
        .unwrap();
    println!("{}", ans);
}
