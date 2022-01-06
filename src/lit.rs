/*
impl ToTokens for LitStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitByteStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitByte {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitChar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitInt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitFloat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.repr.token.to_tokens(tokens);
    }
}

impl ToTokens for LitBool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = if self.value { "true" } else { "false" };
        tokens.append(Ident::new(s, self.span));
    }
}
*/
