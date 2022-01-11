use crate::algorithm::Printer;
use crate::INDENT;
use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};
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
            Expander,
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
                        self.macro_rules_tokens(stream, true);
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
                    self.neverbreak();
                    let stream = group.stream();
                    if !stream.is_empty() {
                        self.cbox(INDENT);
                        self.hardbreak();
                        self.ibox(0);
                        self.macro_rules_tokens(stream, false);
                        self.end();
                        self.hardbreak();
                        self.offset(-INDENT);
                        self.end();
                    }
                    self.word("}");
                    state = Expander;
                }
                (Expander, TokenTree::Punct(punct)) if punct.as_char() == ';' => {
                    self.word(";");
                    self.hardbreak();
                    state = Start;
                }
                _ => unimplemented!("bad macro_rules syntax"),
            }
        }
        match state {
            Start => {}
            Expander => {
                self.word(";");
                self.hardbreak();
            }
            _ => self.hardbreak(),
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    fn macro_rules_tokens(&mut self, stream: TokenStream, matcher: bool) {
        #[derive(PartialEq)]
        enum State {
            Start,
            Dollar,
            DollarIdent,
            DollarIdentColon,
            DollarParen,
            DollarParenSep,
            Pound,
            PoundBang,
            Other,
        }

        use State::*;

        let mut state = Start;
        for token in stream {
            let (needs_space, next_state) = match (&state, &token) {
                (Dollar, TokenTree::Ident(_)) => (false, if matcher { DollarIdent } else { Other }),
                (DollarIdent, TokenTree::Punct(punct))
                    if punct.as_char() == ':' && punct.spacing() == Spacing::Alone =>
                {
                    (false, DollarIdentColon)
                }
                (DollarIdentColon, TokenTree::Ident(_)) => (false, Other),
                (DollarParen, TokenTree::Punct(punct))
                    if (punct.as_char() == '+'
                        || punct.as_char() == '*'
                        || punct.as_char() == '?')
                        && punct.spacing() == Spacing::Alone =>
                {
                    (false, Other)
                }
                (DollarParen, TokenTree::Ident(_)) | (DollarParen, TokenTree::Literal(_)) => {
                    (false, DollarParenSep)
                }
                (DollarParen, TokenTree::Punct(punct)) => match punct.spacing() {
                    Spacing::Joint => (false, DollarParen),
                    Spacing::Alone => (false, DollarParenSep),
                },
                (DollarParenSep, TokenTree::Punct(punct))
                    if punct.as_char() == '+' || punct.as_char() == '*' =>
                {
                    (false, Other)
                }
                (Pound, TokenTree::Punct(punct)) if punct.as_char() == '!' => (false, PoundBang),
                (Start, TokenTree::Group(_)) => (false, Other),
                (Dollar, TokenTree::Group(group))
                    if group.delimiter() == Delimiter::Parenthesis =>
                {
                    (false, DollarParen)
                }
                (Pound, TokenTree::Group(group)) | (PoundBang, TokenTree::Group(group))
                    if group.delimiter() == Delimiter::Bracket =>
                {
                    (false, Other)
                }
                (_, TokenTree::Group(_)) => (true, Other),
                (_, TokenTree::Punct(punct)) if punct.as_char() == ',' => (false, Other),
                (_, TokenTree::Punct(punct)) if punct.as_char() == '$' => (state != Start, Dollar),
                (_, TokenTree::Punct(punct)) if punct.as_char() == '#' => (state != Start, Pound),
                (_, _) => (state != Start, Other),
            };
            if needs_space {
                self.space();
            }
            self.single_token(
                token,
                if matcher {
                    |printer, stream| printer.macro_rules_tokens(stream, true)
                } else {
                    |printer, stream| printer.macro_rules_tokens(stream, false)
                },
            );
            state = next_state;
        }
    }
}
