/*
impl ToTokens for Macro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.bang_token.to_tokens(tokens);
        match &self.delimiter {
            MacroDelimiter::Paren(paren) => {
                paren.surround(tokens, |tokens| self.tokens.to_tokens(tokens));
            }
            MacroDelimiter::Brace(brace) => {
                brace.surround(tokens, |tokens| self.tokens.to_tokens(tokens));
            }
            MacroDelimiter::Bracket(bracket) => {
                bracket.surround(tokens, |tokens| self.tokens.to_tokens(tokens));
            }
        }
    }
}
*/
