// -*- coding:utf-8 -*-

use std::env;

fn double_arg(mut argv: env::Args) -> i32 {
    let arg1 = argv.nth(1).unwrap(); // エラー1
    let n = arg1.parse::<i32>().unwrap(); // エラー2
    2 * n
}

fn main() {
    let n = double_arg(env::args());
    println!("{}", n);
}
