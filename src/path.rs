/*
impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.leading_colon.to_tokens(tokens);
        self.segments.to_tokens(tokens);
    }
}

impl ToTokens for PathSegment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.arguments.to_tokens(tokens);
    }
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(arguments) => {
                arguments.to_tokens(tokens);
            }
            PathArguments::Parenthesized(arguments) => {
                arguments.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for GenericArgument {
    #[allow(clippy::match_same_arms)]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            GenericArgument::Lifetime(lt) => lt.to_tokens(tokens),
            GenericArgument::Type(ty) => ty.to_tokens(tokens),
            GenericArgument::Binding(tb) => tb.to_tokens(tokens),
            GenericArgument::Constraint(tc) => tc.to_tokens(tokens),
            GenericArgument::Const(e) => match *e {
                Expr::Lit(_) => e.to_tokens(tokens),

                // NOTE: We should probably support parsing blocks with only
                // expressions in them without the full feature for const
                // generics.
                Expr::Block(_) => e.to_tokens(tokens),

                // ERROR CORRECTION: Add braces to make sure that the
                // generated code is valid.
                _ => token::Brace::default().surround(tokens, |tokens| {
                    e.to_tokens(tokens);
                }),
            },
        }
    }
}

impl ToTokens for AngleBracketedGenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon2_token.to_tokens(tokens);
        self.lt_token.to_tokens(tokens);

        // Print lifetimes before types and consts, all before bindings,
        // regardless of their order in self.args.
        //
        // TODO: ordering rules for const arguments vs type arguments have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        let mut trailing_or_empty = true;
        for param in self.args.pairs() {
            match **param.value() {
                GenericArgument::Lifetime(_) => {
                    param.to_tokens(tokens);
                    trailing_or_empty = param.punct().is_some();
                }
                GenericArgument::Type(_)
                | GenericArgument::Binding(_)
                | GenericArgument::Constraint(_)
                | GenericArgument::Const(_) => {}
            }
        }
        for param in self.args.pairs() {
            match **param.value() {
                GenericArgument::Type(_) | GenericArgument::Const(_) => {
                    if !trailing_or_empty {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    param.to_tokens(tokens);
                    trailing_or_empty = param.punct().is_some();
                }
                GenericArgument::Lifetime(_)
                | GenericArgument::Binding(_)
                | GenericArgument::Constraint(_) => {}
            }
        }
        for param in self.args.pairs() {
            match **param.value() {
                GenericArgument::Binding(_) | GenericArgument::Constraint(_) => {
                    if !trailing_or_empty {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    param.to_tokens(tokens);
                    trailing_or_empty = param.punct().is_some();
                }
                GenericArgument::Lifetime(_)
                | GenericArgument::Type(_)
                | GenericArgument::Const(_) => {}
            }
        }

        self.gt_token.to_tokens(tokens);
    }
}

impl ToTokens for Binding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for Constraint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for ParenthesizedGenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.inputs.to_tokens(tokens);
        });
        self.output.to_tokens(tokens);
    }
}

impl private {
    pub(crate) fn print_path(tokens: &mut TokenStream, qself: &Option<QSelf>, path: &Path) {
        let qself = match qself {
            Some(qself) => qself,
            None => {
                path.to_tokens(tokens);
                return;
            }
        };
        qself.lt_token.to_tokens(tokens);
        qself.ty.to_tokens(tokens);

        let pos = cmp::min(qself.position, path.segments.len());
        let mut segments = path.segments.pairs();
        if pos > 0 {
            TokensOrDefault(&qself.as_token).to_tokens(tokens);
            path.leading_colon.to_tokens(tokens);
            for (i, segment) in segments.by_ref().take(pos).enumerate() {
                if i + 1 == pos {
                    segment.value().to_tokens(tokens);
                    qself.gt_token.to_tokens(tokens);
                    segment.punct().to_tokens(tokens);
                } else {
                    segment.to_tokens(tokens);
                }
            }
        } else {
            qself.gt_token.to_tokens(tokens);
            path.leading_colon.to_tokens(tokens);
        }
        for segment in segments {
            segment.to_tokens(tokens);
        }
    }
}
*/
