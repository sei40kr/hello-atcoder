use std::cmp;
use std::collections::BTreeMap;
use std::mem;

// median-of-medians.rs --- Median of Medians
// author: Seong Yong-ju <sei40kr@gmail.com>

// tanakah's input macro
// cf https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

fn count_invs(slice: &[i32]) -> (Vec<i32>, u64) {
    let len = slice.len();
    if len <= 1 {
        return (slice.to_vec(), 0);
    }

    let mid = len / 2;
    let (xs, xs_invs) = count_invs(&slice[..mid]);
    let (ys, ys_invs) = count_invs(&slice[mid..]);

    let mut result = Vec::with_capacity(len);
    let mut invs = xs_invs + ys_invs;

    let (mut i, mut j) = (0, 0);
    let (xs_len, ys_len) = (xs.len(), ys.len());
    while i < xs_len && j < ys_len {
        let (x, y) = (xs[i], ys[j]);
        if x <= y {
            result.push(x);
            i += 1;
        } else {
            result.push(y);
            j += 1;
            invs += (xs_len - i) as u64;
        }
    }

    result.extend_from_slice(&xs[i..]);
    result.extend_from_slice(&ys[j..]);

    (result, invs)
}

fn main() {
    input! {
      n: usize,
      xs: [i32; n]
    }

    let ms_len = n as u64 * (n as u64 + 1) / 2;

    let mut l = 0;
    let mut r = 1_000_000_001;
    while r - l > 1 {
        let mid = (l + r) / 2;

        let ts = xs
            .iter()
            .map(|&x| if mid <= x { 1 } else { -1 })
            .collect::<Vec<i32>>();

        let mut ss = vec![0; n + 1];
        for i in 0..n {
            ss[i + 1] += ss[i] + ts[i];
        }

        let invs = count_invs(&ss).1;
        if invs <= ms_len / 2 {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);
}
