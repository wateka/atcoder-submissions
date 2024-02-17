#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use ac_library::{dsu::Dsu, modint::ModInt998244353 as Mint9, modint::Mod1000000007 as Mint10};
use itertools::{Itertools, izip, iproduct};
use superslice::Ext;
use proconio::{fastout, input, marker::{Chars, Usize1}};
macro_rules! yes_no { ($condition: expr) => {println!("{}", if $condition {"Yes"} else {"No"})} }
macro_rules! putln { ($value: expr) => {println!("{}", $value)} }

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut b = vec![0; n+1];
    for (i, ai) in a.into_iter().enumerate() {
        if ai == -1 {
            b[0] = i+1;
        } else {
            b[ai as usize] = i+1;
        }
    }

    let mut next = b[0];
    while next != 0 {
        print!("{} ", next);
        next = b[next];
    }
    println!()
}