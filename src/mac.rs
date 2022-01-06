use crate::unparse::Printer;
use syn::{Macro, MacroDelimiter};

impl Printer {
    pub fn mac(&mut self, mac: &Macro) {
        self.path(&mac.path);
        self.word("!");
        let (open, close) = match mac.delimiter {
            MacroDelimiter::Paren(_) => ('(', ')'),
            MacroDelimiter::Brace(_) => ('{', '}'),
            MacroDelimiter::Bracket(_) => ('[', ']'),
        };
        self.character(open);
        self.tokens(&mac.tokens);
        self.character(close);
    }

    pub fn mac_semi_if_needed(&mut self, delimiter: &MacroDelimiter) {
        match delimiter {
            MacroDelimiter::Paren(_) | MacroDelimiter::Bracket(_) => self.word(";"),
            MacroDelimiter::Brace(_) => {}
        }
    }
}
