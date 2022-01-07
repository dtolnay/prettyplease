#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::match_like_matches_macro,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::unused_self
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

const INDENT: isize = 4;

pub fn unparse(file: &File) -> String {
    let mut p = Printer::new();
    p.file(file);
    p.eof()
}
