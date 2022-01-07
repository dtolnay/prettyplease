#![allow(dead_code)]

use crate::algorithm::{self, BeginToken, BreakToken, Breaks, Printer, Token};
use std::borrow::Cow;

impl Printer {
    // "raw box"
    pub fn rbox(&mut self, indent: usize, b: Breaks) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: b,
        });
    }

    // Inconsistent breaking box
    pub fn ibox(&mut self, indent: usize) {
        self.rbox(indent, Breaks::Inconsistent);
    }

    // Consistent breaking box
    pub fn cbox(&mut self, indent: usize) {
        self.rbox(indent, Breaks::Consistent);
    }

    pub fn break_offset(&mut self, n: usize, off: isize) {
        self.scan_break(BreakToken {
            offset: off,
            blank_space: n as isize,
        });
    }

    pub fn end(&mut self) {
        self.scan_end();
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let s = wrd.into();
        self.scan_string(s);
    }

    fn spaces(&mut self, n: usize) {
        self.break_offset(n, 0);
    }

    pub fn zerobreak(&mut self) {
        self.spaces(0);
    }

    pub fn space(&mut self) {
        self.spaces(1);
    }

    pub fn nbsp(&mut self) {
        self.word(" ");
    }

    pub fn hardbreak(&mut self) {
        self.spaces(algorithm::SIZE_INFINITY as usize);
    }

    pub fn hardbreak_tok_offset(off: isize) -> Token {
        Token::Break(BreakToken {
            offset: off,
            blank_space: algorithm::SIZE_INFINITY,
        })
    }
}
