#![feature(const_fn,plugin)]
#![plugin(stainless)]
#![allow(unused_variables, unused_mut, dead_code)]

extern crate tdd_kata;
#[macro_use(expect)]
extern crate expectest;

mod bowling_kata;
mod string_calc_kata;
mod lexer_sql_kata;
mod stack_kata;
mod queue_kata;
mod map_kata;
