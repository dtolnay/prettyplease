use crate::algorithm::Printer;
use crate::path::PathKind;
use syn::{AttrStyle, Attribute};

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
        if let Some(mut doc) = value_of_attribute("doc", attr) {
            if !doc.contains('\n')
                && match attr.style {
                    AttrStyle::Outer => !doc.starts_with('/'),
                    AttrStyle::Inner(_) => true,
                }
            {
                trim_trailing_spaces(&mut doc);
                self.word(match attr.style {
                    AttrStyle::Outer => "///",
                    AttrStyle::Inner(_) => "//!",
                });
                self.word(doc);
                self.hardbreak();
                return;
            } else if can_be_block_comment(&doc)
                && match attr.style {
                    AttrStyle::Outer => !doc.starts_with(&['*', '/'][..]),
                    AttrStyle::Inner(_) => true,
                }
            {
                trim_interior_trailing_spaces(&mut doc);
                self.word(match attr.style {
                    AttrStyle::Outer => "/**",
                    AttrStyle::Inner(_) => "/*!",
                });
                self.word(doc);
                self.word("*/");
                self.hardbreak();
                return;
            }
        } else if let Some(mut comment) = value_of_attribute("comment", attr) {
            if !comment.contains('\n') {
                trim_trailing_spaces(&mut comment);
                self.word("//");
                self.word(comment);
                self.hardbreak();
                return;
            } else if can_be_block_comment(&comment) && !comment.starts_with(&['*', '!'][..]) {
                trim_interior_trailing_spaces(&mut comment);
                self.word("/*");
                self.word(comment);
                self.word("*/");
                self.hardbreak();
                return;
            }
        }

        self.word(match attr.style {
            AttrStyle::Outer => "#",
            AttrStyle::Inner(_) => "#!",
        });
        self.word("[");
        self.path(&attr.path(), PathKind::Simple);
        self.word("]");
        self.space();
    }
}

fn value_of_attribute(requested: &str, attr: &Attribute) -> Option<String> {
    let is_doc = attr.path().leading_colon.is_none()
        && attr.path().segments.len() == 1
        && attr.path().segments[0].arguments.is_none()
        && attr.path().segments[0].ident == requested;
    if !is_doc {
        return None;
    }
    None
}

pub fn has_outer(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let AttrStyle::Outer = attr.style {
            return true;
        }
    }
    false
}

pub fn has_inner(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let AttrStyle::Inner(_) = attr.style {
            return true;
        }
    }
    false
}

fn trim_trailing_spaces(doc: &mut String) {
    doc.truncate(doc.trim_end_matches(' ').len());
}

fn trim_interior_trailing_spaces(doc: &mut String) {
    if !doc.contains(" \n") {
        return;
    }
    let mut trimmed = String::with_capacity(doc.len());
    let mut lines = doc.split('\n').peekable();
    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            trimmed.push_str(line.trim_end_matches(' '));
            trimmed.push('\n');
        } else {
            trimmed.push_str(line);
        }
    }
    *doc = trimmed;
}

fn can_be_block_comment(value: &str) -> bool {
    let mut depth = 0usize;
    let bytes = value.as_bytes();
    let mut i = 0usize;
    let upper = bytes.len() - 1;

    while i < upper {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
            if depth == 0 {
                return false;
            }
            depth -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    depth == 0
}
