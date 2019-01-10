// lcs.rs --- LCS
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
        s: chars,
        t: chars,
    };

    let (s_len, t_len) = (s.len(), t.len());
    let mut dp = vec![vec![0; t_len + 1]; s_len + 1];

    for i in 1..(s_len + 1) {
        for j in 1..(t_len + 1) {
            dp[i][j] = *[
                dp[i - 1][j - 1] + if s[i - 1] == t[j - 1] { 1 } else { 0 },
                dp[i - 1][j],
                dp[i][j - 1],
            ]
            .into_iter()
            .max()
            .unwrap()
        }
    }

    let (mut i, mut j) = (s_len, t_len);
    let mut ans = String::with_capacity(dp[s_len][t_len]);

    while i != 0 && j != 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else if dp[i][j] == dp[i - 1][j - 1] + 1 {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }

    ans = String::from_utf8(ans.into_bytes().into_iter().rev().collect::<Vec<_>>())
        .ok()
        .unwrap();
    println!("{}", ans);
}
