// -*- coding:utf-8 -*-

use std::env;
use std::error;
use std::fmt;

#[derive(Debug)]
struct NotEnoughArgsError;

impl fmt::Display for NotEnoughArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "引数が不足しています")
    }
}

impl error::Error for NotEnoughArgsError {
    fn description(&self) -> &str {
        "引数が不足しています"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn double_arg(mut argv: env::Args) -> Result<i32, Box<error::Error>> {
    let number_str = try!(argv.nth(1).ok_or(NotEnoughArgsError));
    let n = try!(number_str.parse::<i32>());
    Ok(2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("エラー：{}", err),
    }
}
