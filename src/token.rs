use crate::unparse::Printer;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

impl Printer {
    pub fn tokens(&mut self, tokens: &TokenStream) {
        for token in tokens.clone() {
            match token {
                TokenTree::Group(group) => self.token_group(&group),
                TokenTree::Ident(ident) => self.ident(&ident),
                TokenTree::Punct(punct) => self.token_punct(&punct),
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
        self.tokens(&group.stream());
        self.word(close);
    }

    pub fn ident(&mut self, ident: &Ident) {
        self.word(&ident.to_string());
    }

    pub fn token_punct(&mut self, punct: &Punct) {
        self.character(punct.as_char());
    }

    pub fn token_literal(&mut self, literal: &Literal) {
        self.word(&literal.to_string());
    }
}
