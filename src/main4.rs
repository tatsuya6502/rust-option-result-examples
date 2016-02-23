// -*- coding:utf-8 -*-

use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    let arg1 = try!(argv.nth(1).ok_or("数字を１つ指定してください。".to_owned()));
    let n = try!(arg1.parse::<i32>().map_err(|err| err.to_string()));
    Ok(2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("エラー：{}", err),
    }
}
