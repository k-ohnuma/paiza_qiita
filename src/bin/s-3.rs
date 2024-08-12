#!/usr/bin/env rust-script

use std::{collections::HashMap, io::Read};

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
        a: [usize; n]
    }
    let mut map = HashMap::new();
    let nc3 = |n: usize| {
        if n <= 2 {
            return 0
        }
        n * (n - 1) * (n - 2) / 6
    };
    let nc2 = |n: usize| {
        if n <= 1 {
            return 0
        }
        n * (n - 1) / 2
    };
    for &num in a.iter() {
        map.entry(num % 7).and_modify(|e| *e += 1).or_insert(1usize);
    }
    let mut ans = 0;
    for i in 0..7usize {
        for j in i..7 {
            for k in j..7 {
                if (i + j + k) % 7 != 0 {
                    continue;
                }
                let mut count = HashMap::new();
                count.entry(i).and_modify(|e| *e += 1).or_insert(1);
                count.entry(j).and_modify(|e| *e += 1).or_insert(1);
                count.entry(k).and_modify(|e| *e += 1).or_insert(1usize);
                let mut tmp = 1;

                for (&m, &c) in count.iter() {
                    let &cc = map.get(&m).unwrap_or(&0);
                    if c == 3 {
                        tmp *= nc3(cc);
                    } else if c == 2 {
                        tmp *= nc2(cc);
                    } else {
                        tmp *= cc;
                    }
                }
                ans += tmp;
            }
        }
    }
    println!("{}", ans);
}

