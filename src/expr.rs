/*
// If the given expression is a bare `ExprStruct`, wraps it in parenthesis
// before appending it to `TokenStream`.
fn wrap_bare_struct(tokens: &mut TokenStream, e: &Expr) {
    if let Expr::Struct(_) = *e {
        token::Paren::default().surround(tokens, |tokens| {
            e.to_tokens(tokens);
        });
    } else {
        e.to_tokens(tokens);
    }
}

pub(crate) fn outer_attrs_to_tokens(attrs: &[Attribute], tokens: &mut TokenStream) {
    tokens.append_all(attrs.outer());
}

fn inner_attrs_to_tokens(attrs: &[Attribute], tokens: &mut TokenStream) {
    tokens.append_all(attrs.inner());
}

impl ToTokens for ExprBox {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.box_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.bracket_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            self.elems.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.func.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.receiver.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.method.to_tokens(tokens);
        self.turbofish.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}

impl ToTokens for MethodTurbofish {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.colon2_token.to_tokens(tokens);
        self.lt_token.to_tokens(tokens);
        self.args.to_tokens(tokens);
        self.gt_token.to_tokens(tokens);
    }
}

impl ToTokens for GenericMethodArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            GenericMethodArgument::Type(t) => t.to_tokens(tokens),
            GenericMethodArgument::Const(c) => c.to_tokens(tokens),
        }
    }
}

impl ToTokens for ExprTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.paren_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            self.elems.to_tokens(tokens);
            // If we only have one argument, we need a trailing comma to
            // distinguish ExprTuple from ExprParen.
            if self.elems.len() == 1 && !self.elems.trailing_punct() {
                <Token![,]>::default().to_tokens(tokens);
            }
        });
    }
}

impl ToTokens for ExprBinary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.left.to_tokens(tokens);
        self.op.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.op.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.lit.to_tokens(tokens);
    }
}

impl ToTokens for ExprCast {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.expr.to_tokens(tokens);
        self.as_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for ExprType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.expr.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

fn maybe_wrap_else(tokens: &mut TokenStream, else_: &Option<(Token![else], Box<Expr>)>) {
    if let Some((else_token, else_)) = else_ {
        else_token.to_tokens(tokens);

        // If we are not one of the valid expressions to exist in an else
        // clause, wrap ourselves in a block.
        match **else_ {
            Expr::If(_) | Expr::Block(_) => {
                else_.to_tokens(tokens);
            }
            _ => {
                token::Brace::default().surround(tokens, |tokens| {
                    else_.to_tokens(tokens);
                });
            }
        }
    }
}

impl ToTokens for ExprLet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.let_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
    }
}

impl ToTokens for ExprIf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.if_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.cond);
        self.then_branch.to_tokens(tokens);
        maybe_wrap_else(tokens, &self.else_branch);
    }
}

impl ToTokens for ExprWhile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.label.to_tokens(tokens);
        self.while_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.cond);
        self.body.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            tokens.append_all(&self.body.stmts);
        });
    }
}

impl ToTokens for ExprForLoop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.label.to_tokens(tokens);
        self.for_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.in_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.body.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            tokens.append_all(&self.body.stmts);
        });
    }
}

impl ToTokens for ExprLoop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.label.to_tokens(tokens);
        self.loop_token.to_tokens(tokens);
        self.body.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            tokens.append_all(&self.body.stmts);
        });
    }
}

impl ToTokens for ExprMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.match_token.to_tokens(tokens);
        wrap_bare_struct(tokens, &self.expr);
        self.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            for (i, arm) in self.arms.iter().enumerate() {
                arm.to_tokens(tokens);
                // Ensure that we have a comma after a non-block arm, except
                // for the last one.
                let is_last = i == self.arms.len() - 1;
                if !is_last && requires_terminator(&arm.body) && arm.comma.is_none() {
                    <Token![,]>::default().to_tokens(tokens);
                }
            }
        });
    }
}

impl ToTokens for ExprAsync {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.async_token.to_tokens(tokens);
        self.capture.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl ToTokens for ExprAwait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.base.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.await_token.to_tokens(tokens);
    }
}

impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.try_token.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl ToTokens for ExprYield {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.yield_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.asyncness.to_tokens(tokens);
        self.movability.to_tokens(tokens);
        self.capture.to_tokens(tokens);
        self.or1_token.to_tokens(tokens);
        self.inputs.to_tokens(tokens);
        self.or2_token.to_tokens(tokens);
        self.output.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.unsafe_token.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            tokens.append_all(&self.block.stmts);
        });
    }
}

impl ToTokens for ExprBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.label.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            tokens.append_all(&self.block.stmts);
        });
    }
}

impl ToTokens for ExprAssign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.left.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

impl ToTokens for ExprAssignOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.left.to_tokens(tokens);
        self.op.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

impl ToTokens for ExprField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.base.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.member.to_tokens(tokens);
    }
}

impl ToTokens for Member {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Member::Named(ident) => ident.to_tokens(tokens),
            Member::Unnamed(index) => index.to_tokens(tokens),
        }
    }
}

impl ToTokens for Index {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut lit = Literal::i64_unsuffixed(i64::from(self.index));
        lit.set_span(self.span);
        tokens.append(lit);
    }
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.expr.to_tokens(tokens);
        self.bracket_token.surround(tokens, |tokens| {
            self.index.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.from.to_tokens(tokens);
        match &self.limits {
            RangeLimits::HalfOpen(t) => t.to_tokens(tokens),
            RangeLimits::Closed(t) => t.to_tokens(tokens),
        }
        self.to.to_tokens(tokens);
    }
}

impl ToTokens for ExprPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        private::print_path(tokens, &self.qself, &self.path);
    }
}

impl ToTokens for ExprReference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.and_token.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprBreak {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.break_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprContinue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.continue_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
    }
}

impl ToTokens for ExprReturn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.return_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl ToTokens for ExprMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.mac.to_tokens(tokens);
    }
}

impl ToTokens for ExprStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.path.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            self.fields.to_tokens(tokens);
            if let Some(dot2_token) = &self.dot2_token {
                dot2_token.to_tokens(tokens);
            } else if self.rest.is_some() {
                Token![..](Span::call_site()).to_tokens(tokens);
            }
            self.rest.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.bracket_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
            self.len.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.group_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.paren_token.surround(tokens, |tokens| {
            inner_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
        });
    }
}

impl ToTokens for ExprTry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.expr.to_tokens(tokens);
        self.question_token.to_tokens(tokens);
    }
}

impl ToTokens for Label {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        outer_attrs_to_tokens(&self.attrs, tokens);
        self.member.to_tokens(tokens);
        if let Some(colon_token) = &self.colon_token {
            colon_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }
}

impl ToTokens for Arm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.pat.to_tokens(tokens);
        if let Some((if_token, guard)) = &self.guard {
            if_token.to_tokens(tokens);
            guard.to_tokens(tokens);
        }
        self.fat_arrow_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
        self.comma.to_tokens(tokens);
    }
}
*/
