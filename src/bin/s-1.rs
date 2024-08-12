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
        x: usize,
        a: [usize; n]
    }
    const MOD: usize = 1_000_000_000;
    
    let mut state_table: HashMap<(usize, bool), usize> = HashMap::new();
    state_table.insert((0, false), 1);

    for &num in a.iter() {
        if num == 0 {
            continue;
        }

        let mut new_state_table: HashMap<(usize, bool), usize> = HashMap::new();

        // 前回のゲーム終了時の連勝状態を先頭から順に参照する
        for (&(wins, flag), &count) in &state_table {
            // 連勝数x未満で負けのパターン
            let key = (0, flag);
            let entry = new_state_table.entry(key).or_insert(0);
            *entry = (*entry + count * (num.min(x - wins) as usize)) % MOD;

            if num > x - wins {
                // ボーナスを満たして負けるパターン
                let key = (0, true);
                let entry = new_state_table.entry(key).or_insert(0);
                *entry = (*entry + count) % MOD;
            } else {
                // 連勝で終わるパターン
                let key = (wins + num, flag);
                let entry = new_state_table.entry(key).or_insert(0);
                *entry = (*entry + count) % MOD;
            }
        }

        // state_table を上書きする
        state_table = new_state_table;
    }
    let mut bonus = 0;

    // ボーナス発生パターンを集計
    for (&(wins, flag), &count) in &state_table {
        if flag || wins == x {
            bonus = (bonus + count) % MOD;
        }
    }
    println!("{}", bonus)
}
