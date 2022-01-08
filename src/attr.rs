use crate::algorithm::Printer;
use proc_macro2::TokenTree;
use syn::{AttrStyle, Attribute, Lit, PathArguments};

impl Printer {
    pub fn outer_attrs(&mut self, attrs: &[Attribute]) {
        for attr in attrs {
            if let AttrStyle::Outer = attr.style {
                self.attr(attr);
            }
        }
    }

    pub fn inner_attrs(&mut self, attrs: &[Attribute]) {
        for attr in attrs {
            if let AttrStyle::Inner(_) = attr.style {
                self.attr(attr);
            }
        }
    }

    fn attr(&mut self, attr: &Attribute) {
        if let Some(doc) = doc(attr) {
            if doc.contains('\n') {
                self.word(match attr.style {
                    AttrStyle::Outer => "/**",
                    AttrStyle::Inner(_) => "/*!",
                });
                self.word(doc);
                self.word("*/");
            } else {
                self.word(match attr.style {
                    AttrStyle::Outer => "///",
                    AttrStyle::Inner(_) => "//!",
                });
                self.word(doc);
            }
        } else {
            self.word(match attr.style {
                AttrStyle::Outer => "#",
                AttrStyle::Inner(_) => "#!",
            });
            self.word("[");
            self.path(&attr.path);
            self.tokens(&attr.tokens);
            self.word("]");
        }
        self.hardbreak();
    }
}

fn doc(attr: &Attribute) -> Option<String> {
    let is_doc = attr.path.leading_colon.is_none()
        && attr.path.segments.len() == 1
        && matches!(attr.path.segments[0].arguments, PathArguments::None)
        && attr.path.segments[0].ident == "doc";
    if !is_doc {
        return None;
    }
    let mut tokens = attr.tokens.clone().into_iter();
    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {}
        _ => return None,
    }
    let literal = match tokens.next() {
        Some(TokenTree::Literal(literal)) => literal,
        _ => return None,
    };
    if tokens.next().is_some() {
        return None;
    }
    match Lit::new(literal) {
        Lit::Str(string) => Some(string.value()),
        _ => None,
    }
}
