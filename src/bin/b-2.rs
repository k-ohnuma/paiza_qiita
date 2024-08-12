#!/usr/bin/env rust-script

use std::io::Read;

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

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: [[usize; w]; h],
        l: usize,
        abab: [(usize, usize, usize, usize); l]
    }

    let mut ans = vec![0; n];
    let mut now = 0;

    for &(a1, b1, a2, b2) in abab.iter() {
        let num1 = t[a1 - 1][b1 - 1];
        let num2 = t[a2 - 1][b2 - 1];

        if num1 != num2 {
            now += 1;
            now %= n;
            continue;
        }

        ans[now] += 2usize;
    }

    for &num in ans.iter() {
        println!("{}", num);
    }
}
