#![allow(new_without_default)]

use self::NumberType::{Neither, Inc, Dec};

#[derive(PartialEq, Debug)]
pub enum NumberType {
    Neither,
    Inc,
    Dec
}

#[derive(PartialEq, Eq, Debug)]
enum Type {
    Grow,
    Fall,
    Line
}

#[derive(Default)]
pub struct Checker;

impl Checker {

    pub fn new() -> Checker {
        Checker
    }

    pub fn check(&self, n: usize) -> NumberType {
        let mut prev = 0;
        let mut num = n;
        let mut t = Type::Line;
        while num > 0 {
            let current = num % 10;
            if current > prev {
                if t == Type::Fall {
                    return Neither;
                }
                else if t == Type::Line {
                    t = Type::Grow;
                }
            }
            else if current < prev {
                if t == Type::Grow {
                    return Neither;
                }
                if t == Type::Line {
                    t = Type::Fall;
                }
            }
            num /= 10;
            prev = current;
        }
        match t {
            Type::Grow => Dec,
            Type::Fall => Inc,
            Type::Line => Neither,
        }
    }
}
