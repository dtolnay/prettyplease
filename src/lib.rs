#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::items_after_statements,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::unused_self,
    clippy::vec_init_then_push
)]

mod algorithm;
mod attr;
mod convenience;
mod data;
mod expr;
mod file;
mod generics;
mod item;
mod iter;
mod lifetime;
mod lit;
mod mac;
mod pat;
mod path;
mod ring;
mod stmt;
mod token;
mod ty;

use crate::algorithm::Printer;
use syn::File;

// Target line width.
const MARGIN: isize = 89;

// Number of spaces increment at each level of block indentation.
const INDENT: isize = 4;

// Every line is allowed at least this much space, even if highly indented.
const MIN_SPACE: isize = 60;

pub fn unparse(file: &File) -> String {
    let mut p = Printer::new();
    p.file(file);
    p.eof()
}
