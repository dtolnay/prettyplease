mod algorithm;
mod attr;
mod convenience;
mod data;
mod expr;
mod file;
mod generics;
mod item;
mod lifetime;
mod lit;
mod mac;
mod pat;
mod path;
mod stmt;
mod token;
mod ty;

use crate::algorithm::Printer;
use syn::File;

pub fn unparse(file: &File) -> String {
    let mut p = Printer::new();
    p.file(file);
    p.eof()
}
