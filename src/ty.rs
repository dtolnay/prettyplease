/*
impl ToTokens for TypeSlice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket_token.surround(tokens, |tokens| {
            self.elem.to_tokens(tokens);
        });
    }
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket_token.surround(tokens, |tokens| {
            self.elem.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
            self.len.to_tokens(tokens);
        });
    }
}

impl ToTokens for TypePtr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.star_token.to_tokens(tokens);
        match &self.mutability {
            Some(tok) => tok.to_tokens(tokens),
            None => {
                TokensOrDefault(&self.const_token).to_tokens(tokens);
            }
        }
        self.elem.to_tokens(tokens);
    }
}

impl ToTokens for TypeReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.and_token.to_tokens(tokens);
        self.lifetime.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.elem.to_tokens(tokens);
    }
}

impl ToTokens for TypeBareFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lifetimes.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.inputs.to_tokens(tokens);
            if let Some(variadic) = &self.variadic {
                if !self.inputs.empty_or_trailing() {
                    let span = variadic.dots.spans[0];
                    Token![,](span).to_tokens(tokens);
                }
                variadic.to_tokens(tokens);
            }
        });
        self.output.to_tokens(tokens);
    }
}

impl ToTokens for TypeNever {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bang_token.to_tokens(tokens);
    }
}

impl ToTokens for TypeTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);
        });
    }
}

impl ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        private::print_path(tokens, &self.qself, &self.path);
    }
}

impl ToTokens for TypeTraitObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dyn_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for TypeImplTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for TypeGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.group_token.surround(tokens, |tokens| {
            self.elem.to_tokens(tokens);
        });
    }
}

impl ToTokens for TypeParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.elem.to_tokens(tokens);
        });
    }
}

impl ToTokens for TypeInfer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.underscore_token.to_tokens(tokens);
    }
}

impl ToTokens for TypeMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.mac.to_tokens(tokens);
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ReturnType::Default => {}
            ReturnType::Type(arrow, ty) => {
                arrow.to_tokens(tokens);
                ty.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        if let Some((name, colon)) = &self.name {
            name.to_tokens(tokens);
            colon.to_tokens(tokens);
        }
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.dots.to_tokens(tokens);
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.extern_token.to_tokens(tokens);
        self.name.to_tokens(tokens);
    }
}
*/
