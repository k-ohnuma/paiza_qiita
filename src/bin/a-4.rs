#!/usr/bin/env rust-script

use std::{cmp::max, io::Read};

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
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

fn f(c1: char, c2: char) -> bool {
    if c1 == 'G' && c2 == 'C' || c1 == 'C' && c2 == 'P' || c1 == 'P' && c2 == 'G' {
        return true
    }
    false
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String
    }
    let ss = s.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![None; m + 1]; n + 1];
    dp[0][0] = Some(0usize);

    for i in 0..n {
        for j in 0..m + 1 {
            if dp[i][j].is_none() {
                continue;
            }
            let now = dp[i][j].unwrap();
            match dp[i + 1][j] {
                Some(v) => {
                    let next = if f('G', ss[i]) {1} else {0};
                    dp[i + 1][j] = Some(max(v, now + next));
                },
                None => {
                    let next = if f('G', ss[i]) {1} else {0};
                    dp[i + 1][j] = Some(now + next);
                }
            }
            if j + 2 < m + 1 {
                match dp[i + 1][j + 2] {
                    Some(v) => {
                        let next = if f('C', ss[i]) {1} else {0};
                        dp[i + 1][j + 2] = Some(max(v, now + next));
                    },
                    None => {
                        let next = if f('C', ss[i]) {1} else {0};
                        dp[i + 1][j + 2] = Some(now + next);
                    }
                }
            }
            if j + 5 < m + 1 {
                match dp[i + 1][j + 5] {
                    Some(v) => {
                        let next = if f('P', ss[i]) {1} else {0};
                        dp[i + 1][j + 5] = Some(max(v, now + next));
                    },
                    None => {
                        let next = if f('P', ss[i]) {1} else {0};
                        dp[i + 1][j + 5] = Some(now + next);
                    }
                }
            }
        }
    }
    let ans = dp[n][m].unwrap();
    println!("{}", ans);
}
