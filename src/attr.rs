/*
impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);
        if let AttrStyle::Inner(b) = &self.style {
            b.to_tokens(tokens);
        }
        self.bracket_token.surround(tokens, |tokens| {
            self.path.to_tokens(tokens);
            self.tokens.to_tokens(tokens);
        });
    }
}

impl ToTokens for MetaList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.nested.to_tokens(tokens);
        });
    }
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.lit.to_tokens(tokens);
    }
}
*/
