#!/usr/bin/env rust-script

use std::{
    collections::{HashMap, HashSet, VecDeque},
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

fn f(n: usize, from: usize, target: usize, oths: usize, ans: &mut Vec<(usize, usize)>) {
    if n > 0 {
        f(n - 1, from, oths, target, ans);
        ans.push((from, target));
        f(n - 1, oths, target, from, ans);
    }
}
fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut ans = vec![];
    f(n, 0, 2, 1, &mut ans);
    let mut tower = vec![VecDeque::new(); 3];
    for i in 0..n {
        tower[0].push_back(i);
    }

    for i in 0..k {
        let (from, to) = ans[i];
        let pre = tower[from].pop_front().unwrap();
        tower[to].push_front(pre);
    }
    for row in tower.iter() {
        if row.len() == 0 {
            println!("-");
            continue;
        }
        let mut tmp = row.clone().into_iter().collect::<Vec<usize>>();
        tmp.reverse();
        let ans = tmp.iter().map(|e| (e + 1).to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", ans);
    }

}
