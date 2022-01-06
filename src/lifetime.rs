/*
impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut apostrophe = Punct::new('\'', Spacing::Joint);
        apostrophe.set_span(self.apostrophe);
        tokens.append(apostrophe);
        self.ident.to_tokens(tokens);
    }
}
*/
