use std::collections::BTreeSet;

// meaningful-mean.rs --- Meaningful Mean
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

use std::ops::{AddAssign, Sub};
/// YY is a data structure that can efficiently update elements
/// and calculate prefix sums in a table of numbers.
/// [https://en.wikipedia.org/wiki/Fenwick_tree](https://en.wikipedia.org/wiki/Fenwick_tree)
pub struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    init: T,
}

impl<T: Copy + AddAssign + Sub<Output = T>> FenwickTree<T> {
    /// Constructs a new YY. The size of YY should be specified by YY.
    pub fn new(size: usize, init: T) -> FenwickTree<T> {
        FenwickTree {
            n: size + 1,
            data: vec![init; size + 1],
            init: init,
        }
    }

    pub fn add(&mut self, k: usize, value: T) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= x + 1;
        }
    }

    /// Returns a sum of range YY
    pub fn sum(&self, l: usize, r: usize) -> T {
        self.sum_one(r) - self.sum_one(l)
    }

    /// Returns a sum of range Y
    pub fn sum_one(&self, k: usize) -> T {
        if k >= self.n {
            panic!("");
        }

        let mut result = self.init;
        let mut x = k as i32 - 1;
        while x >= 0 {
            result += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        result
    }
}

fn main() {
    input! {
      n: usize,
      k: i64,
      xs: [i64; n]
    }

    let mut bs = vec![0; n];
    for i in 0..n {
        bs[i] = if 0 < i { bs[i - 1] } else { 0 } + xs[i] - k;
    }

    let mut bs_with_index = (0..n).map(|i: usize| (i, bs[i])).collect::<Vec<_>>();
    bs_with_index.sort_by_key(|&(_, b)| b);

    let mut cs = vec![0; n];
    for i in 0..n {
        let (j, _) = bs_with_index[i];
        cs[i] = j;
    }

    let mut bit = FenwickTree::new(n, 0);
    let mut ans = 0;
    for c in cs {
        ans += bit.sum_one(c + 1);
        bit.add(c, 1);
    }

    for b in bs {
        if 0 <= b {
            ans += 1;
        }
    }

    println!("{}", ans);
}
