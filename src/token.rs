use crate::algorithm::Printer;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

impl Printer {
    pub fn single_token(&mut self, token: TokenTree, group_contents: fn(&mut Self, TokenStream)) {
        match token {
            TokenTree::Group(group) => self.token_group(&group, group_contents),
            TokenTree::Ident(ident) => self.ident(&ident),
            TokenTree::Punct(punct) => self.token_punct(&punct),
            TokenTree::Literal(literal) => self.token_literal(&literal),
        }
    }

    fn token_group(&mut self, group: &Group, group_contents: fn(&mut Self, TokenStream)) {
        let delimiter = group.delimiter();
        self.delimiter_open(delimiter);
        let stream = group.stream();
        if !stream.is_empty() {
            if delimiter == Delimiter::Brace {
                self.space();
            }
            group_contents(self, stream);
            if delimiter == Delimiter::Brace {
                self.space();
            }
        }
        self.delimiter_close(delimiter);
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

    pub fn delimiter_open(&mut self, delimiter: Delimiter) {
        self.word(match delimiter {
            Delimiter::Parenthesis => "(",
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::None => return,
        });
    }

    pub fn delimiter_close(&mut self, delimiter: Delimiter) {
        self.word(match delimiter {
            Delimiter::Parenthesis => ")",
            Delimiter::Brace => "}",
            Delimiter::Bracket => "]",
            Delimiter::None => return,
        });
    }
}
