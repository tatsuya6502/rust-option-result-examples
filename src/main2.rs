// -*- coding:utf-8 -*-

use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    match argv.nth(1) {
        None => Err("数字を１つ指定してください。".to_owned()),
        Some(arg1) => {
            match arg1.parse::<i32>() {
                Ok(n) => Ok(2 * n),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("エラー：{}", err),
    }
}
