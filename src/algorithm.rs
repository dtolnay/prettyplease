// Derived from https://github.com/rust-lang/rust/blob/1.57.0/compiler/rustc_ast_pretty/src/pp.rs

use crate::ring::RingBuffer;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::iter;

// How to break. Described in more detail in the module docs.
#[derive(Clone, Copy, PartialEq)]
pub enum Breaks {
    Consistent,
    Inconsistent,
}

#[derive(Clone, Copy)]
pub struct BreakToken {
    pub offset: isize,
    pub blank_space: isize,
}

#[derive(Clone, Copy)]
pub struct BeginToken {
    pub offset: isize,
    pub breaks: Breaks,
}

#[derive(Clone)]
pub enum Token {
    // In practice a string token contains either a `&'static str` or a
    // `String`. `Cow` is overkill for this because we never modify the data,
    // but it's more convenient than rolling our own more specialized type.
    String(Cow<'static, str>),
    Break(BreakToken),
    Begin(BeginToken),
    End,
}

#[derive(Copy, Clone)]
enum PrintFrame {
    Fits,
    Broken(isize, Breaks),
}

pub const SIZE_INFINITY: isize = 0xffff;

pub struct Printer {
    out: String,
    // Width of lines we're constrained to
    margin: isize,
    // Number of spaces left on line
    space: isize,
    // Ring-buffer of tokens and calculated sizes
    buf: RingBuffer<BufEntry>,
    // Running size of stream "...left"
    left_total: isize,
    // Running size of stream "...right"
    right_total: isize,
    // Pseudo-stack, really a ring too. Holds the primary-ring-buffers index of
    // the Begin that started the current block, possibly with the most recent
    // Break after that Begin (if there is any) on top of it. Stuff is flushed
    // off the bottom as it becomes irrelevant due to the primary ring-buffer
    // advancing.
    scan_stack: VecDeque<usize>,
    // Stack of blocks-in-progress being flushed by print
    print_stack: Vec<PrintFrame>,
    // Buffered indentation to avoid writing trailing whitespace
    pending_indentation: isize,
}

#[derive(Clone)]
struct BufEntry {
    token: Token,
    size: isize,
}

impl Printer {
    pub fn new() -> Self {
        let linewidth = 78;
        Printer {
            out: String::new(),
            margin: linewidth as isize,
            space: linewidth as isize,
            buf: RingBuffer::new(),
            left_total: 0,
            right_total: 0,
            scan_stack: VecDeque::new(),
            print_stack: Vec::new(),
            pending_indentation: 0,
        }
    }

    pub fn eof(mut self) -> String {
        if !self.scan_stack.is_empty() {
            self.check_stack(0);
            self.advance_left();
        }
        self.out
    }

    pub fn scan_begin(&mut self, token: BeginToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.buf.clear();
        }
        let right = self.buf.push(BufEntry {
            token: Token::Begin(token),
            size: -self.right_total,
        });
        self.scan_stack.push_back(right);
    }

    pub fn scan_end(&mut self) {
        if self.scan_stack.is_empty() {
            self.print_end();
        } else {
            let right = self.buf.push(BufEntry {
                token: Token::End,
                size: -1,
            });
            self.scan_stack.push_back(right);
        }
    }

    pub fn scan_break(&mut self, token: BreakToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.buf.clear();
        } else {
            self.check_stack(0);
        }
        let right = self.buf.push(BufEntry {
            token: Token::Break(token),
            size: -self.right_total,
        });
        self.scan_stack.push_back(right);
        self.right_total += token.blank_space;
    }

    pub fn scan_string(&mut self, string: Cow<'static, str>) {
        if self.scan_stack.is_empty() {
            self.print_string(string);
        } else {
            let len = string.len() as isize;
            self.buf.push(BufEntry {
                token: Token::String(string),
                size: len,
            });
            self.right_total += len;
            self.check_stream();
        }
    }

    fn check_stream(&mut self) {
        while self.right_total - self.left_total > self.space {
            if *self.scan_stack.front().unwrap() == self.buf.index_of_first() {
                self.scan_stack.pop_front().unwrap();
                self.buf.first_mut().size = SIZE_INFINITY;
            }
            self.advance_left();
        }
    }

    fn advance_left(&mut self) {
        while self.buf.first().size >= 0 {
            let left = self.buf.pop_first();

            match left.token {
                Token::String(string) => {
                    self.left_total += string.len() as isize;
                    self.print_string(string);
                }
                Token::Break(token) => {
                    self.left_total += token.blank_space;
                    self.print_break(token, left.size);
                }
                Token::Begin(token) => self.print_begin(token, left.size),
                Token::End => self.print_end(),
            }

            if self.buf.is_empty() {
                break;
            }
        }
    }

    fn check_stack(&mut self, mut depth: usize) {
        while let Some(&index) = self.scan_stack.back() {
            let mut entry = &mut self.buf[index];
            match entry.token {
                Token::Begin(_) => {
                    if depth == 0 {
                        break;
                    }
                    self.scan_stack.pop_back().unwrap();
                    entry.size += self.right_total;
                    depth -= 1;
                }
                Token::End => {
                    self.scan_stack.pop_back().unwrap();
                    entry.size = 1;
                    depth += 1;
                }
                _ => {
                    self.scan_stack.pop_back().unwrap();
                    entry.size += self.right_total;
                    if depth == 0 {
                        break;
                    }
                }
            }
        }
    }

    fn get_top(&self) -> PrintFrame {
        const OUTER: PrintFrame = PrintFrame::Broken(0, Breaks::Inconsistent);
        self.print_stack.last().map_or(OUTER, PrintFrame::clone)
    }

    fn print_begin(&mut self, token: BeginToken, size: isize) {
        if size > self.space {
            let col = self.margin - self.space + token.offset;
            self.print_stack.push(PrintFrame::Broken(col, token.breaks));
        } else {
            self.print_stack.push(PrintFrame::Fits);
        }
    }

    fn print_end(&mut self) {
        self.print_stack.pop().unwrap();
    }

    fn print_break(&mut self, token: BreakToken, size: isize) {
        if let Some(offset) = match self.get_top() {
            PrintFrame::Fits => None,
            PrintFrame::Broken(offset, Breaks::Consistent) => Some(offset),
            PrintFrame::Broken(offset, Breaks::Inconsistent) => {
                if size > self.space {
                    Some(offset)
                } else {
                    None
                }
            }
        } {
            self.out.push('\n');
            self.pending_indentation = offset + token.offset;
            self.space = self.margin - self.pending_indentation;
        } else {
            self.pending_indentation += token.blank_space;
            self.space -= token.blank_space;
        }
    }

    fn print_string(&mut self, string: Cow<'static, str>) {
        self.out.reserve(self.pending_indentation as usize);
        self.out
            .extend(iter::repeat(' ').take(self.pending_indentation as usize));
        self.pending_indentation = 0;

        self.out.push_str(&string);
        self.space -= string.len() as isize;
    }
}
