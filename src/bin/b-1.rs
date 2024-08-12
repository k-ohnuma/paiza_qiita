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

fn transpose<T: Clone + Copy + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![T::default(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: [[String; x + 1]; z]
    }
    let mut a = vec![];
    for row in s.iter() {
        let mut v = vec![];
        for word in row.iter() {
            v.push(word.chars().collect::<Vec<char>>());
        }
        v.pop();
        a.push(v);
    }
    let mut ans = vec![];
    for i in 0..a.len() {
        let mut v = vec![];
        let trans_a = transpose(a[i].clone());
        for j in 0..trans_a.len() {
            if trans_a[j].contains(&'#') {
                v.push('#');
            } else {
                v.push('.');
            }
        }
        ans.push(v);
    }
    ans.reverse();
    for row in ans.iter() {
        let w = row.iter().collect::<String>();
        println!("{}", w);
    }
}
