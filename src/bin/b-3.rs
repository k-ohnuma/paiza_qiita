#!/usr/bin/env rust-script

use std::{collections::HashSet, io::Read};

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
        k: usize,
        m: usize,
        d: [String; k],
        s: [String; m]
    }

    let mut db = d.into_iter().collect::<HashSet<String>>();
    let mut ss = vec![];
    for word in s.iter() {
        let chars = word.chars().collect::<Vec<char>>();
        ss.push(chars);
    }
    let mut start = ss[0][0];
    let mut players = (0..n).collect::<Vec<usize>>();
    let mut player = 0;
    for (idx, s_v) in ss.iter().enumerate() {
        if s_v[0] != start {
            players.remove(player);
            if idx + 1 < m {
                start = ss[idx + 1][0];
            }
            continue;
        }
        if *s_v.last().unwrap() == 'z' {
            players.remove(player);
            if idx + 1 < m {
                start = ss[idx + 1][0];
            }
            continue;
        }
        let word = s_v.iter().collect::<String>();
        if !db.contains(&word) {
            players.remove(player);
            if idx + 1 < m {
                start = ss[idx + 1][0];
            }
            continue;
        }
        db.remove(&word);
        player += 1;
        player %= players.len();
        start = *s_v.last().unwrap();
    }
    println!("{}", players.len());
    for &num in players.iter() {
        println!("{}", num + 1);
    }
}
