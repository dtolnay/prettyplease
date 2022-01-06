/*
impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(&self.stmts);
        });
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Stmt::Local(local) => local.to_tokens(tokens),
            Stmt::Item(item) => item.to_tokens(tokens),
            Stmt::Expr(expr) => expr.to_tokens(tokens),
            Stmt::Semi(expr, semi) => {
                expr.to_tokens(tokens);
                semi.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for Local {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        expr::printing::outer_attrs_to_tokens(&self.attrs, tokens);
        self.let_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        if let Some((eq_token, init)) = &self.init {
            eq_token.to_tokens(tokens);
            init.to_tokens(tokens);
        }
        self.semi_token.to_tokens(tokens);
    }
}
*/
