use crate::algorithm::Printer;
use crate::iter::IterDelimited;
use crate::path::PathKind;
use crate::INDENT;
use syn::token::Pub;
use syn::{Field, Fields, FieldsUnnamed, Variant, VisRestricted, Visibility};

impl Printer {
    pub fn variant(&mut self, variant: &Variant) {
        self.outer_attrs(&variant.attrs);
        self.ident(&variant.ident);
        match &variant.fields {
            Fields::Named(fields) => {
                self.nbsp();
                self.word("{");
                self.cbox(INDENT);
                self.space();
                for field in fields.named.iter().delimited() {
                    self.field(&field);
                    self.trailing_comma_or_space(field.is_last);
                }
                self.offset(-INDENT);
                self.end();
                self.word("}");
            }
            Fields::Unnamed(fields) => {
                self.cbox(INDENT);
                self.fields_unnamed(fields);
                self.end();
            }
            Fields::Unit => {}
        }
        if let Some((_eq_token, discriminant)) = &variant.discriminant {
            self.word(" = ");
            self.expr(discriminant);
        }
    }

    pub fn fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        self.word("(");
        self.zerobreak();
        for field in fields.unnamed.iter().delimited() {
            self.field(&field);
            self.trailing_comma(field.is_last);
        }
        self.offset(-INDENT);
        self.word(")");
    }

    pub fn field(&mut self, field: &Field) {
        self.outer_attrs(&field.attrs);
        self.visibility(&field.vis);
        if let Some(ident) = &field.ident {
            self.ident(ident);
            self.word(": ");
        }
        self.ty(&field.ty);
    }

    pub fn visibility(&mut self, vis: &Visibility) {
        match vis {
            Visibility::Public(vis) => self.vis_public(vis),
            Visibility::Restricted(vis) => self.vis_restricted(vis),
            Visibility::Inherited => {}
        }
    }

    fn vis_public(&mut self, vis: &Pub) {
        let _ = vis;
        self.word("pub ");
    }

    fn vis_restricted(&mut self, vis: &VisRestricted) {
        self.word("pub(");
        let omit_in = vis.path.leading_colon.is_none()
            && vis.path.segments.len() == 1
            && vis.path.segments[0].arguments.is_none()
            && matches!(
                vis.path.segments[0].ident.to_string().as_str(),
                "self" | "super" | "crate",
            );
        if !omit_in {
            self.word("in ");
        }
        self.path(&vis.path, PathKind::Simple);
        self.word(") ");
    }
}
