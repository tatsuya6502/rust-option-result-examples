// -*- coding:utf-8 -*-

use std::env;
use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
enum CliError {
    NotEnoughArgs,
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::NotEnoughArgs => write!(f, "引数が不足しています"),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::NotEnoughArgs => "引数が不足しています",
            CliError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::NotEnoughArgs => None,
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn double_arg(mut argv: env::Args) -> Result<i32, CliError> {
    let arg1 = try!(argv.nth(1).ok_or(CliError::NotEnoughArgs));
    let n = try!(arg1.parse::<i32>());
    Ok(2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err @ CliError::NotEnoughArgs) => println!("エラー：{}", err),
        Err(CliError::Parse(..)) => {
            println!("エラー：不正な数字です \"{}\"",
                     env::args().nth(1).unwrap())
        }
    }
}
