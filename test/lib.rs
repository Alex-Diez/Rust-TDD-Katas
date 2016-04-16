#![feature(const_fn,plugin)]
#![plugin(stainless)]
#![allow(unused_variables, unused_mut, dead_code)]

extern crate tdd_kata;
#[macro_use(expect)]
extern crate expectest;
pub extern crate rand;

mod bowling_kata;
mod string_calc_kata;
mod lexer_sql_kata;
mod stack_kata;
mod queue_kata;
mod map_kata;
mod lcd_kata;
mod thread_counting_kata;
mod inc_dec_numbers_kata;
mod sorting_kata;
mod bst_kata;
mod graph_search_kata;
mod directed_graph_kata;
mod mst_kata;
