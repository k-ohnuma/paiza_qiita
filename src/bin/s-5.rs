#!/usr/bin/env rust-script

use std::{cmp::Reverse, collections::BinaryHeap, io::Read};

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
        n: usize,
        m: usize,
        q: usize,
        abc: [(usize, usize, usize); m],
        query: [(char, usize); q]
    }
    let mut abc = abc.clone();
    abc.sort_by_key(|&e| Reverse(e.2));

    let mut flg = vec![false; n];
    for &(op, num) in query.iter() {
        let target = num - 1;
        if op == '+' {
            flg[target] = true;
        } else {
            flg[target] = false;
        }
        let mut tmp = false;
        for &(a, b, c) in abc.iter() {

            if flg[a - 1] != flg[b - 1] {
                println!("{}", c);
                tmp = true;
                break;
            }
        }
        if !tmp {
            println!("0");
        }
    }
}
