use crate::algorithm::Printer;
use crate::INDENT;
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed, Variant, VisCrate, VisPublic, VisRestricted,
    Visibility,
};

impl Printer {
    pub fn variant(&mut self, variant: &Variant) {
        self.outer_attrs(&variant.attrs);
        self.ident(&variant.ident);
        self.fields(&variant.fields);
        if let Some((_eq_token, discriminant)) = &variant.discriminant {
            self.nbsp();
            self.word("=");
            self.nbsp();
            self.expr(discriminant);
        }
    }

    fn fields(&mut self, fields: &Fields) {
        match fields {
            Fields::Named(fields) => self.fields_named(fields),
            Fields::Unnamed(fields) => self.fields_unnamed(fields),
            Fields::Unit => {}
        }
    }

    pub fn fields_named(&mut self, fields: &FieldsNamed) {
        self.nbsp();
        self.word("{");
        if !fields.named.is_empty() {
            self.cbox(INDENT);
            self.hardbreak();
            for field in &fields.named {
                self.field(field);
                self.word(",");
                self.hardbreak();
            }
            self.offset(-INDENT);
            self.end();
        }
        self.word("}");
    }

    pub fn fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for field in &fields.unnamed {
            self.field(field);
            self.word(",");
            self.space();
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    fn field(&mut self, field: &Field) {
        self.outer_attrs(&field.attrs);
        self.visibility(&field.vis);
        if let Some(ident) = &field.ident {
            self.ident(ident);
            self.word(":");
            self.nbsp();
        }
        self.ty(&field.ty);
    }

    pub fn visibility(&mut self, vis: &Visibility) {
        match vis {
            Visibility::Public(vis) => self.vis_public(vis),
            Visibility::Crate(vis) => self.vis_crate(vis),
            Visibility::Restricted(vis) => self.vis_restricted(vis),
            Visibility::Inherited => {}
        }
    }

    fn vis_public(&mut self, vis: &VisPublic) {
        let _ = vis;
        self.word("pub");
        self.nbsp();
    }

    fn vis_crate(&mut self, vis: &VisCrate) {
        let _ = vis;
        self.word("crate");
        self.nbsp();
    }

    fn vis_restricted(&mut self, vis: &VisRestricted) {
        self.word("pub(");
        // TODO: If we have a path which is not "self" or "super" or "crate",
        // automatically add the "in" token.
        if vis.in_token.is_some() {
            self.word("in");
            self.nbsp();
        }
        self.path(&vis.path);
        self.word(")");
        self.nbsp();
    }
}
