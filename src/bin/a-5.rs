#!/usr/bin/env rust-script

use std::{cmp::{max, min}, io::Read};

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
        x: usize,
        xx: [usize; n]
    }
    
    let mut dp = vec![vec![None; x + 1]; n + 1];
    dp[0][0] = Some(0usize);

    for i in 0..n {
        for j in 0..x + 1 {
            if dp[i][j].is_none() {
                continue;
            }
            let now = dp[i][j].unwrap();
            let num = xx[i];
            match dp[i + 1][j] {
                Some(v) => {
                    dp[i + 1][j] = Some(max(v, now));
                },
                None => {
                    dp[i + 1][j] = Some(now);
                }
            }
            if j + num < x + 1 {
                match dp[i + 1][j + num] {
                    Some(v) => {
                        dp[i + 1][j + num] = Some(max(v, now + 1));
                    },
                    None => {
                        dp[i + 1][j + num] = Some(now + 1);
                    }
                }
            }
        }
    }

    let mut count = 0;
    let mut ans = usize::MAX;

    for (i, num) in dp[n].iter().enumerate() {
        if let Some(v) = num {
            if *v > count {
                count = *v;
                ans = x - i;
            } else if *v == count {
                ans = min(ans, x - i);
            }
        }
    }
    println!("{}", ans);
}
