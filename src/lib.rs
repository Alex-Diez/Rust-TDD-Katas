#![feature(plugin,core_intrinsics,alloc,shared)]
#![plugin(regex_macros,clippy)]
#![allow(regex_macro,expl_impl_clone_on_copy)]

extern crate regex;
extern crate alloc;

pub mod bowling_kata;
pub mod string_calc_kata;
pub mod lexer_sql_kata;
pub mod stack_kata;
pub mod queue_kata;
pub mod map_kata;
pub mod lcd_kata;
pub mod thread_counting_kata;
pub mod inc_dec_numbers_kata;
pub mod sorting_kata;
