use crate::algorithm::Printer;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

impl Printer {
    pub fn tokens(&mut self, tokens: &TokenStream) {
        self.tokens_owned(tokens.clone());
    }

    fn tokens_owned(&mut self, tokens: TokenStream) {
        let mut previous_is_joint = true;
        for token in tokens {
            if !previous_is_joint {
                match &token {
                    TokenTree::Punct(punct) if punct.as_char() == ',' => {}
                    _ => self.space(),
                }
            }
            previous_is_joint = false;
            match token {
                TokenTree::Group(group) => self.token_group(&group),
                TokenTree::Ident(ident) => self.ident(&ident),
                TokenTree::Punct(punct) => {
                    previous_is_joint = punct.spacing() == Spacing::Joint;
                    self.token_punct(&punct);
                }
                TokenTree::Literal(literal) => self.token_literal(&literal),
            }
        }
    }

    pub fn token_group(&mut self, group: &Group) {
        let (open, close) = match group.delimiter() {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };
        self.word(open);
        self.tokens_owned(group.stream());
        self.word(close);
    }

    pub fn ident(&mut self, ident: &Ident) {
        self.word(ident.to_string());
    }

    pub fn token_punct(&mut self, punct: &Punct) {
        self.word(punct.as_char().to_string());
    }

    pub fn token_literal(&mut self, literal: &Literal) {
        self.word(literal.to_string());
    }
}
