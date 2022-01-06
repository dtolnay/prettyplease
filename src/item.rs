/*
impl ToTokens for ItemExternCrate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.extern_token.to_tokens(tokens);
        self.crate_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        if let Some((as_token, rename)) = &self.rename {
            as_token.to_tokens(tokens);
            rename.to_tokens(tokens);
        }
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemUse {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.use_token.to_tokens(tokens);
        self.leading_colon.to_tokens(tokens);
        self.tree.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemStatic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.static_token.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.const_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.block.stmts);
        });
    }
}

impl ToTokens for ItemMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.mod_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        if let Some((brace, items)) = &self.content {
            brace.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(items);
            });
        } else {
            TokensOrDefault(&self.semi).to_tokens(tokens);
        }
    }
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.abi.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.items);
        });
    }
}

impl ToTokens for ItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.enum_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.variants.to_tokens(tokens);
        });
    }
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.struct_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        match &self.fields {
            Fields::Named(fields) => {
                self.generics.where_clause.to_tokens(tokens);
                fields.to_tokens(tokens);
            }
            Fields::Unnamed(fields) => {
                fields.to_tokens(tokens);
                self.generics.where_clause.to_tokens(tokens);
                TokensOrDefault(&self.semi_token).to_tokens(tokens);
            }
            Fields::Unit => {
                self.generics.where_clause.to_tokens(tokens);
                TokensOrDefault(&self.semi_token).to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.union_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.fields.to_tokens(tokens);
    }
}

impl ToTokens for ItemTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.auto_token.to_tokens(tokens);
        self.trait_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        if !self.supertraits.is_empty() {
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
            self.supertraits.to_tokens(tokens);
        }
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.items);
        });
    }
}

impl ToTokens for ItemTraitAlias {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.trait_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.defaultness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.impl_token.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        if let Some((polarity, path, for_token)) = &self.trait_ {
            polarity.to_tokens(tokens);
            path.to_tokens(tokens);
            for_token.to_tokens(tokens);
        }
        self.self_ty.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.items);
        });
    }
}

impl ToTokens for ItemMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.mac.path.to_tokens(tokens);
        self.mac.bang_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        match &self.mac.delimiter {
            MacroDelimiter::Paren(paren) => {
                paren.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
            }
            MacroDelimiter::Brace(brace) => {
                brace.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
            }
            MacroDelimiter::Bracket(bracket) => {
                bracket.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
            }
        }
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemMacro2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.macro_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.rules.to_tokens(tokens);
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.colon2_token.to_tokens(tokens);
        self.tree.to_tokens(tokens);
    }
}

impl ToTokens for UseName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.as_token.to_tokens(tokens);
        self.rename.to_tokens(tokens);
    }
}

impl ToTokens for UseGlob {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.star_token.to_tokens(tokens);
    }
}

impl ToTokens for UseGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            self.items.to_tokens(tokens);
        });
    }
}

impl ToTokens for TraitItemConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.const_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        if let Some((eq_token, default)) = &self.default {
            eq_token.to_tokens(tokens);
            default.to_tokens(tokens);
        }
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for TraitItemMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.sig.to_tokens(tokens);
        match &self.default {
            Some(block) => {
                block.brace_token.surround(tokens, |tokens| {
                    tokens.append_all(self.attrs.inner());
                    tokens.append_all(&block.stmts);
                });
            }
            None => {
                TokensOrDefault(&self.semi_token).to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        if !self.bounds.is_empty() {
            TokensOrDefault(&self.colon_token).to_tokens(tokens);
            self.bounds.to_tokens(tokens);
        }
        self.generics.where_clause.to_tokens(tokens);
        if let Some((eq_token, default)) = &self.default {
            eq_token.to_tokens(tokens);
            default.to_tokens(tokens);
        }
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for TraitItemMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.mac.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ImplItemConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.defaultness.to_tokens(tokens);
        self.const_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ImplItemMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.defaultness.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        if self.block.stmts.len() == 1 {
            if let Stmt::Item(Item::Verbatim(verbatim)) = &self.block.stmts[0] {
                if verbatim.to_string() == ";" {
                    verbatim.to_tokens(tokens);
                    return;
                }
            }
        }
        self.block.brace_token.surround(tokens, |tokens| {
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.block.stmts);
        });
    }
}

impl ToTokens for ImplItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.defaultness.to_tokens(tokens);
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ImplItemMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.mac.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ForeignItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.static_token.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ForeignItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.vis.to_tokens(tokens);
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ForeignItemMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        self.mac.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

fn maybe_variadic_to_tokens(arg: &FnArg, tokens: &mut TokenStream) -> bool {
    let arg = match arg {
        FnArg::Typed(arg) => arg,
        FnArg::Receiver(receiver) => {
            receiver.to_tokens(tokens);
            return false;
        }
    };

    match arg.ty.as_ref() {
        Type::Verbatim(ty) if ty.to_string() == "..." => {
            match arg.pat.as_ref() {
                Pat::Verbatim(pat) if pat.to_string() == "..." => {
                    tokens.append_all(arg.attrs.outer());
                    pat.to_tokens(tokens);
                }
                _ => arg.to_tokens(tokens),
            }
            true
        }
        _ => {
            arg.to_tokens(tokens);
            false
        }
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.constness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            let mut last_is_variadic = false;
            for input in self.inputs.pairs() {
                match input {
                    Pair::Punctuated(input, comma) => {
                        maybe_variadic_to_tokens(input, tokens);
                        comma.to_tokens(tokens);
                    }
                    Pair::End(input) => {
                        last_is_variadic = maybe_variadic_to_tokens(input, tokens);
                    }
                }
            }
            if self.variadic.is_some() && !last_is_variadic {
                if !self.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.variadic.to_tokens(tokens);
            }
        });
        self.output.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.attrs.outer());
        if let Some((ampersand, lifetime)) = &self.reference {
            ampersand.to_tokens(tokens);
            lifetime.to_tokens(tokens);
        }
        self.mutability.to_tokens(tokens);
        self.self_token.to_tokens(tokens);
    }
}
*/
