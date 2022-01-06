/*
impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.ident.to_tokens(tokens);
        self.fields.to_tokens(tokens);
        if let Some((eq_token, disc)) = &self.discriminant {
            eq_token.to_tokens(tokens);
            disc.to_tokens(tokens);
        }
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            self.named.to_tokens(tokens);
        });
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.unnamed.to_tokens(tokens);
        });
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        if let Some(ident) = &self.ident {
            ident.to_tokens(tokens);
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
        }
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for VisPublic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pub_token.to_tokens(tokens);
    }
}

impl ToTokens for VisCrate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.crate_token.to_tokens(tokens);
    }
}

impl ToTokens for VisRestricted {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pub_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            // TODO: If we have a path which is not "self" or "super" or
            // "crate", automatically add the "in" token.
            self.in_token.to_tokens(tokens);
            self.path.to_tokens(tokens);
        });
    }
}
*/
