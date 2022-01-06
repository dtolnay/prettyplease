// Derived from https://github.com/rust-lang/rust/blob/1.57.0/compiler/rustc_ast_pretty/src/pp.rs

use std::borrow::Cow;
use std::collections::VecDeque;

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
    Eof,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::Eof)
    }

    pub fn is_hardbreak_tok(&self) -> bool {
        matches!(
            self,
            Token::Break(BreakToken {
                offset: 0,
                blank_space: SIZE_INFINITY,
            })
        )
    }
}

#[derive(Copy, Clone)]
enum PrintStackBreak {
    Fits,
    Broken(Breaks),
}

#[derive(Copy, Clone)]
struct PrintStackElem {
    offset: isize,
    pbreak: PrintStackBreak,
}

pub const SIZE_INFINITY: isize = 0xffff;

pub fn mk_printer() -> Printer {
    let linewidth = 78;
    // Yes 55, it makes the ring buffers big enough to never fall behind.
    let n: usize = 55 * linewidth;
    Printer {
        out: String::new(),
        buf_max_len: n,
        margin: linewidth as isize,
        space: linewidth as isize,
        left: 0,
        right: 0,
        // Initialize a single entry; advance_right() will extend it on demand
        // up to `buf_max_len` elements.
        buf: vec![BufEntry::default()],
        left_total: 0,
        right_total: 0,
        scan_stack: VecDeque::new(),
        print_stack: Vec::new(),
        pending_indentation: 0,
    }
}

pub struct Printer {
    out: String,
    buf_max_len: usize,
    // Width of lines we're constrained to
    margin: isize,
    // Number of spaces left on line
    space: isize,
    // Index of left side of input stream
    left: usize,
    // Index of right side of input stream
    right: usize,
    // Ring-buffer of tokens and calculated sizes
    buf: Vec<BufEntry>,
    // Running size of stream "...left"
    left_total: isize,
    // Running size of stream "...right"
    right_total: isize,
    // Pseudo-stack, really a ring too. Holds the
    // primary-ring-buffers index of the Begin that started the
    // current block, possibly with the most recent Break after that
    // Begin (if there is any) on top of it. Stuff is flushed off the
    // bottom as it becomes irrelevant due to the primary ring-buffer
    // advancing.
    scan_stack: VecDeque<usize>,
    // Stack of blocks-in-progress being flushed by print
    print_stack: Vec<PrintStackElem>,
    // Buffered indentation to avoid writing trailing whitespace
    pending_indentation: isize,
}

#[derive(Clone)]
struct BufEntry {
    token: Token,
    size: isize,
}

impl Default for BufEntry {
    fn default() -> Self {
        BufEntry {
            token: Token::Eof,
            size: 0,
        }
    }
}

impl Printer {
    pub fn last_token(&self) -> Token {
        self.buf[self.right].token.clone()
    }

    // Be very careful with this!
    pub fn replace_last_token(&mut self, t: Token) {
        self.buf[self.right].token = t;
    }

    pub fn eof(mut self) -> String {
        if !self.scan_stack.is_empty() {
            self.check_stack(0);
            self.advance_left();
        }
        self.out
    }

    pub fn scan_begin(&mut self, b: BeginToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.left = 0;
            self.right = 0;
        } else {
            self.advance_right();
        }
        self.scan_push(BufEntry {
            token: Token::Begin(b),
            size: -self.right_total,
        });
    }

    pub fn scan_end(&mut self) {
        if self.scan_stack.is_empty() {
            self.print_end();
        } else {
            self.advance_right();
            self.scan_push(BufEntry {
                token: Token::End,
                size: -1,
            });
        }
    }

    pub fn scan_break(&mut self, b: BreakToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.left = 0;
            self.right = 0;
        } else {
            self.advance_right();
        }
        self.check_stack(0);
        self.scan_push(BufEntry {
            token: Token::Break(b),
            size: -self.right_total,
        });
        self.right_total += b.blank_space;
    }

    pub fn scan_string(&mut self, s: Cow<'static, str>) {
        if self.scan_stack.is_empty() {
            self.print_string(s);
        } else {
            self.advance_right();
            let len = s.len() as isize;
            self.buf[self.right] = BufEntry {
                token: Token::String(s),
                size: len,
            };
            self.right_total += len;
            self.check_stream();
        }
    }

    fn check_stream(&mut self) {
        if self.right_total - self.left_total > self.space {
            if Some(&self.left) == self.scan_stack.back() {
                let scanned = self.scan_pop_bottom();
                self.buf[scanned].size = SIZE_INFINITY;
            }
            self.advance_left();
            if self.left != self.right {
                self.check_stream();
            }
        }
    }

    fn scan_push(&mut self, entry: BufEntry) {
        self.buf[self.right] = entry;
        self.scan_stack.push_front(self.right);
    }

    fn scan_pop(&mut self) -> usize {
        self.scan_stack.pop_front().unwrap()
    }

    fn scan_top(&self) -> usize {
        *self.scan_stack.front().unwrap()
    }

    fn scan_pop_bottom(&mut self) -> usize {
        self.scan_stack.pop_back().unwrap()
    }

    fn advance_right(&mut self) {
        self.right += 1;
        self.right %= self.buf_max_len;
        // Extend the buf if necessary.
        if self.right == self.buf.len() {
            self.buf.push(BufEntry::default());
        }
        assert_ne!(self.right, self.left);
    }

    fn advance_left(&mut self) {
        let mut left_size = self.buf[self.left].size;

        while left_size >= 0 {
            let left = self.buf[self.left].token.clone();

            let len = match left {
                Token::Break(b) => b.blank_space,
                Token::String(ref s) => {
                    let len = s.len() as isize;
                    assert_eq!(len, left_size);
                    len
                }
                _ => 0,
            };

            self.print(left, left_size);

            self.left_total += len;

            if self.left == self.right {
                break;
            }

            self.left += 1;
            self.left %= self.buf_max_len;

            left_size = self.buf[self.left].size;
        }
    }

    fn check_stack(&mut self, k: usize) {
        if !self.scan_stack.is_empty() {
            let x = self.scan_top();
            match self.buf[x].token {
                Token::Begin(_) => {
                    if k > 0 {
                        self.scan_pop();
                        self.buf[x].size += self.right_total;
                        self.check_stack(k - 1);
                    }
                }
                Token::End => {
                    // paper says + not =, but that makes no sense.
                    self.scan_pop();
                    self.buf[x].size = 1;
                    self.check_stack(k + 1);
                }
                _ => {
                    self.scan_pop();
                    self.buf[x].size += self.right_total;
                    if k > 0 {
                        self.check_stack(k);
                    }
                }
            }
        }
    }

    fn print_newline(&mut self, amount: isize) {
        self.out.push('\n');
        self.pending_indentation = 0;
        self.indent(amount);
    }

    fn indent(&mut self, amount: isize) {
        self.pending_indentation += amount;
    }

    fn get_top(&self) -> PrintStackElem {
        *self.print_stack.last().unwrap_or({
            &PrintStackElem {
                offset: 0,
                pbreak: PrintStackBreak::Broken(Breaks::Inconsistent),
            }
        })
    }

    fn print_begin(&mut self, b: BeginToken, l: isize) {
        if l > self.space {
            let col = self.margin - self.space + b.offset;
            self.print_stack.push(PrintStackElem {
                offset: col,
                pbreak: PrintStackBreak::Broken(b.breaks),
            });
        } else {
            self.print_stack.push(PrintStackElem {
                offset: 0,
                pbreak: PrintStackBreak::Fits,
            });
        }
    }

    fn print_end(&mut self) {
        self.print_stack.pop().unwrap();
    }

    fn print_break(&mut self, b: BreakToken, l: isize) {
        let top = self.get_top();
        match top.pbreak {
            PrintStackBreak::Fits => {
                self.space -= b.blank_space;
                self.indent(b.blank_space);
            }
            PrintStackBreak::Broken(Breaks::Consistent) => {
                self.print_newline(top.offset + b.offset);
                self.space = self.margin - (top.offset + b.offset);
            }
            PrintStackBreak::Broken(Breaks::Inconsistent) => {
                if l > self.space {
                    self.print_newline(top.offset + b.offset);
                    self.space = self.margin - (top.offset + b.offset);
                } else {
                    self.indent(b.blank_space);
                    self.space -= b.blank_space;
                }
            }
        }
    }

    fn print_string(&mut self, s: Cow<'static, str>) {
        let len = s.len() as isize;
        // assert!(len <= space);
        self.space -= len;

        // Write the pending indent. A more concise way of doing this would be:
        //
        //   write!(self.out, "{: >n$}", "", n = self.pending_indentation as usize)?;
        //
        // But that is significantly slower. This code is sufficiently hot, and indents can get
        // sufficiently large, that the difference is significant on some workloads.
        self.out.reserve(self.pending_indentation as usize);
        self.out
            .extend(std::iter::repeat(' ').take(self.pending_indentation as usize));
        self.pending_indentation = 0;
        self.out.push_str(&s);
    }

    fn print(&mut self, token: Token, l: isize) {
        match token {
            Token::Begin(b) => self.print_begin(b, l),
            Token::End => self.print_end(),
            Token::Break(b) => self.print_break(b, l),
            Token::String(s) => {
                let len = s.len() as isize;
                assert_eq!(len, l);
                self.print_string(s);
            }
            Token::Eof => panic!(), // Eof should never get here.
        }
    }
}
