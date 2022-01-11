use crate::algorithm::Printer;
use crate::INDENT;
use syn::{Ident, Macro, MacroDelimiter};

impl Printer {
    pub fn mac(&mut self, mac: &Macro, ident: Option<&Ident>) {
        self.path(&mac.path);
        self.word("!");
        if let Some(ident) = ident {
            self.nbsp();
            self.ident(ident);
        }
        let (open, close) = match mac.delimiter {
            MacroDelimiter::Paren(_) => ("(", ")"),
            MacroDelimiter::Brace(_) => (" {", "}"),
            MacroDelimiter::Bracket(_) => ("[", "]"),
        };
        self.word(open);
        self.cbox(INDENT);
        self.zerobreak();
        self.ibox(0);
        self.tokens(&mac.tokens);
        self.end();
        self.zerobreak();
        self.offset(-INDENT);
        self.end();
        self.word(close);
    }

    pub fn mac_semi_if_needed(&mut self, delimiter: &MacroDelimiter) {
        match delimiter {
            MacroDelimiter::Paren(_) | MacroDelimiter::Bracket(_) => self.word(";"),
            MacroDelimiter::Brace(_) => {}
        }
    }
}
