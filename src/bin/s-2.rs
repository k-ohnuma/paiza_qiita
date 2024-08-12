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


pub struct Node {
    cnt: usize,
    next: HashMap<char, Node>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            cnt: 0,
            next: HashMap::new(),
        }
    }

    pub fn contain(&self, key: char) -> bool {
        self.next.contains_key(&key)
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&mut self, seq: &Vec<char>) {
        let mut node = &mut self.root;
        node.cnt += 1;
        for &s in seq.iter() {
            node = node.next.entry(s).or_insert_with(Node::new);
            node.cnt += 1;
        }
        node = node.next.entry(0 as char).or_insert_with(Node::new);
        node.cnt += 1;
    }

    pub fn search(&self, seq: &Vec<char>) -> bool {
        let mut node = &self.root;
        for &s in seq.iter() {
            if let Some(next) = node.next.get(&s) {
                node = next;
            } else {
                return false;
            }
        }
        node.next.contains_key(&(0 as char))
    }

}

fn main() {
    input! {
        n: usize,
        m: usize, 
        sp: [(String, usize); n],
        q: [String; m]
    }
    let mut trie = Trie::new();
    for word in q.iter() {
        let chars = word.chars().collect::<Vec<char>>();
        trie.insert(&chars);
    }
    let mut map = HashMap::new();
    for (s, p) in sp.iter() {
        let mut ans = 0;
        for i in 0..s.len() {
            let chars = &s[..=i].chars().collect::<Vec<char>>();
            if trie.search(&chars) {
                let word = chars.iter().collect::<String>();
                map.entry(word).and_modify(|e | *e += p).or_insert(*p);
            }
        }
    }
    for word in q.iter() {
        let &ans = map.get(word).unwrap_or(&0);
        println!("{}", ans);
    }
    
}
