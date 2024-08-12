#!/usr/bin/env rust-script

use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

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

fn dfs(n: usize) -> (Vec<char>, HashSet<usize>) {
    let mut map = HashSet::new();
    if n == 1 {
        map.insert(0);
        return (vec!['0'], map);
    }
    let (pre, idxs) = dfs(n - 1);
    let mut ans = vec![];
    let mut now = 0;
    for i in 0..pre.len() {
        if idxs.contains(&i) {
            ans.push('0');
            map.insert(now);
            now += 1;
            ans.push(pre[i]);
            now += 1;
            ans.push('1');
            map.insert(now);
            now += 1;
        } else {
            ans.push(pre[i]);
            now += 1;
        }
    }
    (ans, map)
}

fn main() {
    input! {
        n: usize
    }
    let (ans, _) = dfs(n);
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
