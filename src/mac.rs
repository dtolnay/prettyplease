use crate::algorithm::Printer;
use crate::INDENT;
use proc_macro2::{Spacing, TokenStream, TokenTree};
use syn::{Ident, Macro, MacroDelimiter, PathArguments};

impl Printer {
    pub fn mac(&mut self, mac: &Macro, ident: Option<&Ident>) {
        let is_macro_rules = mac.path.leading_colon.is_none()
            && mac.path.segments.len() == 1
            && matches!(mac.path.segments[0].arguments, PathArguments::None)
            && mac.path.segments[0].ident == "macro_rules";
        if is_macro_rules {
            if let Some(ident) = ident {
                self.macro_rules(ident, &mac.tokens);
                return;
            }
        }
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

    fn macro_rules(&mut self, name: &Ident, rules: &TokenStream) {
        enum State {
            Start,
            Matcher,
            Equal,
            Greater,
            Rule,
        }

        use State::*;

        self.word("macro_rules! ");
        self.ident(name);
        self.word(" {");
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        let mut state = State::Start;
        for token in rules.clone() {
            match (state, token) {
                (Start, TokenTree::Group(group)) => {
                    let delimiter = group.delimiter();
                    self.delimiter_open(delimiter);
                    let stream = group.stream();
                    if !stream.is_empty() {
                        self.cbox(INDENT);
                        self.zerobreak();
                        self.ibox(0);
                        self.tokens_owned(stream);
                        self.end();
                        self.zerobreak();
                        self.offset(-INDENT);
                        self.end();
                    }
                    self.delimiter_close(delimiter);
                    state = Matcher;
                }
                (Matcher, TokenTree::Punct(punct))
                    if punct.as_char() == '=' && punct.spacing() == Spacing::Joint =>
                {
                    self.word(" =");
                    state = Equal;
                }
                (Equal, TokenTree::Punct(punct))
                    if punct.as_char() == '>' && punct.spacing() == Spacing::Alone =>
                {
                    self.word(">");
                    state = Greater;
                }
                (Greater, TokenTree::Group(group)) => {
                    self.word(" {");
                    let stream = group.stream();
                    if !stream.is_empty() {
                        self.cbox(INDENT);
                        self.hardbreak();
                        self.ibox(0);
                        self.tokens_owned(stream);
                        self.end();
                        self.hardbreak();
                        self.offset(-INDENT);
                        self.end();
                    }
                    self.word("}");
                    state = Rule;
                }
                (Rule, TokenTree::Punct(punct)) if punct.as_char() == ';' => {
                    self.word(";");
                    self.hardbreak();
                    state = Start;
                }
                _ => unimplemented!("bad macro_rules syntax"),
            }
        }
        match state {
            Start => {}
            Rule => {
                self.word(";");
                self.hardbreak();
            }
            _ => self.hardbreak(),
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }
}
