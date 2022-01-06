/*
impl ToTokens for PatWild {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.underscore_token.to_tokens(tokens);
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.by_ref.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        if let Some((at_token, subpat)) = &self.subpat {
            at_token.to_tokens(tokens);
            subpat.to_tokens(tokens);
        }
    }
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.path.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.fields.to_tokens(tokens);
            // NOTE: We need a comma before the dot2 token if it is present.
            if !self.fields.empty_or_trailing() && self.dot2_token.is_some() {
                <Token![,]>::default().to_tokens(tokens);
            }
            self.dot2_token.to_tokens(tokens);
        });
    }
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.path.to_tokens(tokens);
        self.pat.to_tokens(tokens);
    }
}

impl ToTokens for PatType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for PatPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        private::print_path(tokens, &self.qself, &self.path);
    }
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.paren_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);
        });
    }
}

impl ToTokens for PatBox {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.box_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
    }
}

impl ToTokens for PatReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.and_token.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.pat.to_tokens(tokens);
    }
}

impl ToTokens for PatRest {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.dot2_token.to_tokens(tokens);
    }
}

impl ToTokens for PatLit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for PatRange {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.lo.to_tokens(tokens);
        match &self.limits {
            RangeLimits::HalfOpen(t) => t.to_tokens(tokens),
            RangeLimits::Closed(t) => t.to_tokens(tokens),
        }
        self.hi.to_tokens(tokens);
    }
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.bracket_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);
        });
    }
}

impl ToTokens for PatMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.mac.to_tokens(tokens);
    }
}

impl ToTokens for PatOr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.leading_vert.to_tokens(tokens);
        self.cases.to_tokens(tokens);
    }
}

impl ToTokens for FieldPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        if let Some(colon_token) = &self.colon_token {
            self.member.to_tokens(tokens);
            colon_token.to_tokens(tokens);
        }
        self.pat.to_tokens(tokens);
    }
}
*/
