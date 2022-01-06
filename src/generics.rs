/*
impl ToTokens for Generics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.params.is_empty() {
            return;
        }

        TokensOrDefault(&self.lt_token).to_tokens(tokens);

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        //
        // TODO: ordering rules for const parameters vs type parameters have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        let mut trailing_or_empty = true;
        for param in self.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                param.to_tokens(tokens);
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in self.params.pairs() {
            match **param.value() {
                GenericParam::Type(_) | GenericParam::Const(_) => {
                    if !trailing_or_empty {
                        <Token![,]>::default().to_tokens(tokens);
                        trailing_or_empty = true;
                    }
                    param.to_tokens(tokens);
                }
                GenericParam::Lifetime(_) => {}
            }
        }

        TokensOrDefault(&self.gt_token).to_tokens(tokens);
    }
}

impl<'a> ToTokens for ImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0.params.is_empty() {
            return;
        }

        TokensOrDefault(&self.0.lt_token).to_tokens(tokens);

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        //
        // TODO: ordering rules for const parameters vs type parameters have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        let mut trailing_or_empty = true;
        for param in self.0.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                param.to_tokens(tokens);
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in self.0.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                continue;
            }
            if !trailing_or_empty {
                <Token![,]>::default().to_tokens(tokens);
                trailing_or_empty = true;
            }
            match *param.value() {
                GenericParam::Lifetime(_) => unreachable!(),
                GenericParam::Type(param) => {
                    // Leave off the type parameter defaults
                    tokens.append_all(param.attrs.outer());
                    param.ident.to_tokens(tokens);
                    if !param.bounds.is_empty() {
                        TokensOrDefault(&param.colon_token).to_tokens(tokens);
                        param.bounds.to_tokens(tokens);
                    }
                }
                GenericParam::Const(param) => {
                    // Leave off the const parameter defaults
                    tokens.append_all(param.attrs.outer());
                    param.const_token.to_tokens(tokens);
                    param.ident.to_tokens(tokens);
                    param.colon_token.to_tokens(tokens);
                    param.ty.to_tokens(tokens);
                }
            }
            param.punct().to_tokens(tokens);
        }

        TokensOrDefault(&self.0.gt_token).to_tokens(tokens);
    }
}

impl<'a> ToTokens for TypeGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0.params.is_empty() {
            return;
        }

        TokensOrDefault(&self.0.lt_token).to_tokens(tokens);

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        //
        // TODO: ordering rules for const parameters vs type parameters have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        let mut trailing_or_empty = true;
        for param in self.0.params.pairs() {
            if let GenericParam::Lifetime(def) = *param.value() {
                // Leave off the lifetime bounds and attributes
                def.lifetime.to_tokens(tokens);
                param.punct().to_tokens(tokens);
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in self.0.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                continue;
            }
            if !trailing_or_empty {
                <Token![,]>::default().to_tokens(tokens);
                trailing_or_empty = true;
            }
            match *param.value() {
                GenericParam::Lifetime(_) => unreachable!(),
                GenericParam::Type(param) => {
                    // Leave off the type parameter defaults
                    param.ident.to_tokens(tokens);
                }
                GenericParam::Const(param) => {
                    // Leave off the const parameter defaults
                    param.ident.to_tokens(tokens);
                }
            }
            param.punct().to_tokens(tokens);
        }

        TokensOrDefault(&self.0.gt_token).to_tokens(tokens);
    }
}

impl<'a> ToTokens for Turbofish<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.params.is_empty() {
            <Token![::]>::default().to_tokens(tokens);
            TypeGenerics(self.0).to_tokens(tokens);
        }
    }
}

impl ToTokens for BoundLifetimes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.for_token.to_tokens(tokens);
        self.lt_token.to_tokens(tokens);
        self.lifetimes.to_tokens(tokens);
        self.gt_token.to_tokens(tokens);
    }
}

impl ToTokens for LifetimeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.lifetime.to_tokens(tokens);
        if !self.bounds.is_empty() {
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
            self.bounds.to_tokens(tokens);
        }
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.ident.to_tokens(tokens);
        if !self.bounds.is_empty() {
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
            self.bounds.to_tokens(tokens);
        }
        if let Some(default) = &self.default {
            if self.eq_token.is_none() {
                if let Type::Verbatim(default) = default {
                    let mut iter = default.clone().into_iter().peekable();
                    while let Some(token) = iter.next() {
                        if let TokenTree::Punct(q) = token {
                            if q.as_char() == '~' {
                                if let Some(TokenTree::Ident(c)) = iter.peek() {
                                    if c == "const" {
                                        if self.bounds.is_empty() {
                                            TokensOrDefault(&self.colon_token)
                                                .to_tokens(tokens);
                                        }
                                        return default.to_tokens(tokens);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            TokensOrDefault(&self.eq_token).to_tokens(tokens);
            default.to_tokens(tokens);
        }
    }
}

impl ToTokens for TraitBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let to_tokens = |tokens: &mut TokenStream| {
            let skip = match self.path.segments.pairs().next() {
                Some(Pair::Punctuated(t, p)) if t.ident == "const" => {
                    Token![~](p.spans[0]).to_tokens(tokens);
                    t.to_tokens(tokens);
                    1
                }
                _ => 0,
            };
            self.modifier.to_tokens(tokens);
            self.lifetimes.to_tokens(tokens);
            self.path.leading_colon.to_tokens(tokens);
            tokens.append_all(self.path.segments.pairs().skip(skip));
        };
        match &self.paren_token {
            Some(paren) => paren.surround(tokens, to_tokens),
            None => to_tokens(tokens),
        }
    }
}

impl ToTokens for TraitBoundModifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TraitBoundModifier::None => {}
            TraitBoundModifier::Maybe(t) => t.to_tokens(tokens),
        }
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.const_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        if let Some(default) = &self.default {
            TokensOrDefault(&self.eq_token).to_tokens(tokens);
            default.to_tokens(tokens);
        }
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.predicates.is_empty() {
            self.where_token.to_tokens(tokens);
            self.predicates.to_tokens(tokens);
        }
    }
}

impl ToTokens for PredicateType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lifetimes.to_tokens(tokens);
        self.bounded_ty.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for PredicateLifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lifetime.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for PredicateEq {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lhs_ty.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.rhs_ty.to_tokens(tokens);
    }
}
*/
