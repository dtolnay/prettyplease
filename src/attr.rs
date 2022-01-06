use crate::algorithm::Printer;
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
        self.word(match attr.style {
            AttrStyle::Outer => "#",
            AttrStyle::Inner(_) => "#!",
        });
        self.word("[");
        self.path(&attr.path);
        self.tokens(&attr.tokens);
        self.word("]");
    }
}
