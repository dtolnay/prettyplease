use crate::algorithm::Printer;
use crate::iter::IterDelimited;
use crate::INDENT;
use proc_macro2::TokenStream;
use syn::{
    Arm, BinOp, Expr, ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait, ExprBinary,
    ExprBlock, ExprBox, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField,
    ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch,
    ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn,
    ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprWhile,
    ExprYield, FieldValue, GenericMethodArgument, Index, Label, Member, MethodTurbofish,
    RangeLimits, UnOp,
};

impl Printer {
    pub fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Array(expr) => self.expr_array(expr),
            Expr::Assign(expr) => self.expr_assign(expr),
            Expr::AssignOp(expr) => self.expr_assign_op(expr),
            Expr::Async(expr) => self.expr_async(expr),
            Expr::Await(expr) => self.expr_await(expr),
            Expr::Binary(expr) => self.expr_binary(expr),
            Expr::Block(expr) => self.expr_block(expr),
            Expr::Box(expr) => self.expr_box(expr),
            Expr::Break(expr) => self.expr_break(expr),
            Expr::Call(expr) => self.expr_call(expr),
            Expr::Cast(expr) => self.expr_cast(expr),
            Expr::Closure(expr) => self.expr_closure(expr),
            Expr::Continue(expr) => self.expr_continue(expr),
            Expr::Field(expr) => self.expr_field(expr),
            Expr::ForLoop(expr) => self.expr_for_loop(expr),
            Expr::Group(expr) => self.expr_group(expr),
            Expr::If(expr) => self.expr_if(expr),
            Expr::Index(expr) => self.expr_index(expr),
            Expr::Let(expr) => self.expr_let(expr),
            Expr::Lit(expr) => self.expr_lit(expr),
            Expr::Loop(expr) => self.expr_loop(expr),
            Expr::Macro(expr) => self.expr_macro(expr),
            Expr::Match(expr) => self.expr_match(expr),
            Expr::MethodCall(expr) => self.expr_method_call(expr),
            Expr::Paren(expr) => self.expr_paren(expr),
            Expr::Path(expr) => self.expr_path(expr),
            Expr::Range(expr) => self.expr_range(expr),
            Expr::Reference(expr) => self.expr_reference(expr),
            Expr::Repeat(expr) => self.expr_repeat(expr),
            Expr::Return(expr) => self.expr_return(expr),
            Expr::Struct(expr) => self.expr_struct(expr),
            Expr::Try(expr) => self.expr_try(expr),
            Expr::TryBlock(expr) => self.expr_try_block(expr),
            Expr::Tuple(expr) => self.expr_tuple(expr),
            Expr::Type(expr) => self.expr_type(expr),
            Expr::Unary(expr) => self.expr_unary(expr),
            Expr::Unsafe(expr) => self.expr_unsafe(expr),
            Expr::Verbatim(expr) => self.expr_verbatim(expr),
            Expr::While(expr) => self.expr_while(expr),
            Expr::Yield(expr) => self.expr_yield(expr),
            #[cfg(test)]
            Expr::__TestExhaustive(_) => unreachable!(),
            #[cfg(not(test))]
            _ => unimplemented!("unknown Expr"),
        }
    }

    // If the given expression is a bare `ExprStruct`, wraps it in parenthesis
    // before appending it to `TokenStream`.
    fn wrap_bare_struct(&mut self, expr: &Expr) {
        if let Expr::Struct(_) = expr {
            self.word("(");
            self.expr(expr);
            self.word(")");
        } else {
            self.expr(expr);
        }
    }

    fn expr_array(&mut self, expr: &ExprArray) {
        self.outer_attrs(&expr.attrs);
        self.word("[");
        self.inner_attrs(&expr.attrs);
        for element in &expr.elems {
            self.expr(element);
            self.word(",");
        }
        self.word("]");
    }

    fn expr_assign(&mut self, expr: &ExprAssign) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.left);
        self.word("=");
        self.expr(&expr.right);
    }

    fn expr_assign_op(&mut self, expr: &ExprAssignOp) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.left);
        self.binary_operator(&expr.op);
        self.expr(&expr.right);
    }

    fn expr_async(&mut self, expr: &ExprAsync) {
        self.outer_attrs(&expr.attrs);
        self.word("async ");
        if expr.capture.is_some() {
            self.word("move ");
        }
        self.block(&expr.block);
    }

    fn expr_await(&mut self, expr: &ExprAwait) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.base);
        self.word(".await");
    }

    fn expr_binary(&mut self, expr: &ExprBinary) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.left);
        self.binary_operator(&expr.op);
        self.expr(&expr.right);
    }

    pub fn expr_block(&mut self, expr: &ExprBlock) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("{");
        self.cbox(INDENT);
        self.hardbreak();
        self.inner_attrs(&expr.attrs);
        for stmt in &expr.block.stmts {
            self.stmt(stmt);
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    fn expr_box(&mut self, expr: &ExprBox) {
        self.outer_attrs(&expr.attrs);
        self.word("box");
        self.expr(&expr.expr);
    }

    fn expr_break(&mut self, expr: &ExprBreak) {
        self.outer_attrs(&expr.attrs);
        self.word("break");
        if let Some(lifetime) = &expr.label {
            self.lifetime(lifetime);
        }
        if let Some(value) = &expr.expr {
            self.expr(value);
        }
    }

    fn expr_call(&mut self, expr: &ExprCall) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.func);
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for arg in expr.args.iter().delimited() {
            self.expr(&arg);
            self.trailing_comma(arg.is_last);
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    fn expr_cast(&mut self, expr: &ExprCast) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word("as");
        self.ty(&expr.ty);
    }

    fn expr_closure(&mut self, expr: &ExprClosure) {
        self.outer_attrs(&expr.attrs);
        if expr.asyncness.is_some() {
            self.word("async");
        }
        if expr.movability.is_some() {
            self.word("static");
        }
        if expr.capture.is_some() {
            self.word("move");
        }
        self.word("|");
        for pat in &expr.inputs {
            self.pat(pat);
            self.word(",");
        }
        self.word("|");
        self.return_type(&expr.output);
        self.expr(&expr.body);
    }

    fn expr_continue(&mut self, expr: &ExprContinue) {
        self.outer_attrs(&expr.attrs);
        self.word("continue");
        if let Some(lifetime) = &expr.label {
            self.lifetime(lifetime);
        }
    }

    fn expr_field(&mut self, expr: &ExprField) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.base);
        self.word(".");
        self.member(&expr.member);
    }

    fn expr_for_loop(&mut self, expr: &ExprForLoop) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("for");
        self.pat(&expr.pat);
        self.word("in");
        self.wrap_bare_struct(&expr.expr);
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for stmt in &expr.body.stmts {
            self.stmt(stmt);
        }
        self.word("}");
    }

    fn expr_group(&mut self, expr: &ExprGroup) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr);
    }

    fn expr_if(&mut self, expr: &ExprIf) {
        self.outer_attrs(&expr.attrs);
        self.word("if");
        self.wrap_bare_struct(&expr.cond);
        self.block(&expr.then_branch);
        if let Some((_else_token, else_branch)) = &expr.else_branch {
            self.word("else");
            self.maybe_wrap_else(else_branch);
        }
    }

    fn expr_index(&mut self, expr: &ExprIndex) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word("[");
        self.expr(&expr.index);
        self.word("]");
    }

    fn expr_let(&mut self, expr: &ExprLet) {
        self.outer_attrs(&expr.attrs);
        self.word("let");
        self.pat(&expr.pat);
        self.word("=");
        self.wrap_bare_struct(&expr.expr);
    }

    pub fn expr_lit(&mut self, expr: &ExprLit) {
        self.outer_attrs(&expr.attrs);
        self.lit(&expr.lit);
    }

    fn expr_loop(&mut self, expr: &ExprLoop) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("loop");
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for stmt in &expr.body.stmts {
            self.stmt(stmt);
        }
        self.word("}");
    }

    fn expr_macro(&mut self, expr: &ExprMacro) {
        self.outer_attrs(&expr.attrs);
        self.mac(&expr.mac);
    }

    fn expr_match(&mut self, expr: &ExprMatch) {
        self.outer_attrs(&expr.attrs);
        self.word("match");
        self.wrap_bare_struct(&expr.expr);
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for arm in &expr.arms {
            self.arm(arm);
            if requires_terminator(&arm.body) {
                self.word(",");
            }
        }
        self.word("}");
    }

    fn expr_method_call(&mut self, expr: &ExprMethodCall) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.receiver);
        self.word(".");
        self.ident(&expr.method);
        if let Some(turbofish) = &expr.turbofish {
            self.method_turbofish(turbofish);
        }
        self.word("(");
        for arg in &expr.args {
            self.expr(arg);
            self.word(",");
        }
        self.word(")");
    }

    fn expr_paren(&mut self, expr: &ExprParen) {
        self.outer_attrs(&expr.attrs);
        self.word("(");
        self.inner_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word(")");
    }

    fn expr_path(&mut self, expr: &ExprPath) {
        self.outer_attrs(&expr.attrs);
        self.qpath(&expr.qself, &expr.path);
    }

    fn expr_range(&mut self, expr: &ExprRange) {
        self.outer_attrs(&expr.attrs);
        if let Some(from) = &expr.from {
            self.expr(from);
        }
        self.word(match expr.limits {
            RangeLimits::HalfOpen(_) => "..",
            RangeLimits::Closed(_) => "..=",
        });
        if let Some(to) = &expr.to {
            self.expr(to);
        }
    }

    fn expr_reference(&mut self, expr: &ExprReference) {
        self.outer_attrs(&expr.attrs);
        self.word("&");
        if expr.mutability.is_some() {
            self.word("mut");
        }
        self.expr(&expr.expr);
    }

    fn expr_repeat(&mut self, expr: &ExprRepeat) {
        self.outer_attrs(&expr.attrs);
        self.word("[");
        self.inner_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word(";");
        self.expr(&expr.len);
        self.word("]");
    }

    fn expr_return(&mut self, expr: &ExprReturn) {
        self.outer_attrs(&expr.attrs);
        self.word("return");
        if let Some(value) = &expr.expr {
            self.expr(value);
        }
    }

    fn expr_struct(&mut self, expr: &ExprStruct) {
        self.outer_attrs(&expr.attrs);
        self.path(&expr.path);
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for field_value in &expr.fields {
            self.field_value(field_value);
            self.word(",");
        }
        if let Some(rest) = &expr.rest {
            self.word("..");
            self.expr(rest);
        }
        self.word("}");
    }

    fn expr_try(&mut self, expr: &ExprTry) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word("?");
    }

    fn expr_try_block(&mut self, expr: &ExprTryBlock) {
        self.outer_attrs(&expr.attrs);
        self.word("try");
        self.block(&expr.block);
    }

    fn expr_tuple(&mut self, expr: &ExprTuple) {
        self.outer_attrs(&expr.attrs);
        self.word("(");
        self.inner_attrs(&expr.attrs);
        for elem in &expr.elems {
            self.expr(elem);
            self.word(",");
        }
        self.word(")");
    }

    fn expr_type(&mut self, expr: &ExprType) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr);
        self.word(":");
        self.ty(&expr.ty);
    }

    fn expr_unary(&mut self, expr: &ExprUnary) {
        self.outer_attrs(&expr.attrs);
        self.unary_operator(&expr.op);
        self.expr(&expr.expr);
    }

    fn expr_unsafe(&mut self, expr: &ExprUnsafe) {
        self.outer_attrs(&expr.attrs);
        self.word("unsafe");
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for stmt in &expr.block.stmts {
            self.stmt(stmt);
        }
        self.word("}");
    }

    fn expr_verbatim(&mut self, expr: &TokenStream) {
        if !expr.is_empty() {
            unimplemented!("Expr::Verbatim `{}`", expr);
        }
    }

    fn expr_while(&mut self, expr: &ExprWhile) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("while");
        self.wrap_bare_struct(&expr.cond);
        self.word("{");
        self.inner_attrs(&expr.attrs);
        for stmt in &expr.body.stmts {
            self.stmt(stmt);
        }
        self.word("}");
    }

    fn expr_yield(&mut self, expr: &ExprYield) {
        self.outer_attrs(&expr.attrs);
        self.word("yield");
        if let Some(value) = &expr.expr {
            self.expr(value);
        }
    }

    fn label(&mut self, label: &Label) {
        self.lifetime(&label.name);
        self.word(":");
    }

    fn field_value(&mut self, field_value: &FieldValue) {
        self.outer_attrs(&field_value.attrs);
        self.member(&field_value.member);
        if field_value.colon_token.is_some() {
            self.word(":");
            self.expr(&field_value.expr);
        }
    }

    fn arm(&mut self, arm: &Arm) {
        self.outer_attrs(&arm.attrs);
        self.pat(&arm.pat);
        if let Some((_if_token, guard)) = &arm.guard {
            self.word("if");
            self.expr(guard);
        }
        self.word("=>");
        self.expr(&arm.body);
    }

    fn method_turbofish(&mut self, turbofish: &MethodTurbofish) {
        self.word("::<");
        for arg in &turbofish.args {
            self.generic_method_argument(arg);
        }
        self.word(">");
    }

    fn generic_method_argument(&mut self, generic: &GenericMethodArgument) {
        match generic {
            GenericMethodArgument::Type(arg) => self.ty(arg),
            GenericMethodArgument::Const(arg) => self.expr(arg),
        }
    }

    fn maybe_wrap_else(&mut self, expr: &Expr) {
        // If not one of the valid expressions to exist in an else clause, wrap
        // in a block.
        match expr {
            Expr::If(_) | Expr::Block(_) => {
                self.expr(expr);
            }
            _ => {
                self.word("{");
                self.expr(expr);
                self.word("}");
            }
        }
    }

    pub fn member(&mut self, member: &Member) {
        match member {
            Member::Named(ident) => self.ident(ident),
            Member::Unnamed(index) => self.index(index),
        }
    }

    fn index(&mut self, member: &Index) {
        self.word(member.index.to_string());
    }

    fn binary_operator(&mut self, op: &BinOp) {
        self.word(match op {
            BinOp::Add(_) => "+",
            BinOp::Sub(_) => "-",
            BinOp::Mul(_) => "*",
            BinOp::Div(_) => "/",
            BinOp::Rem(_) => "%",
            BinOp::And(_) => "&&",
            BinOp::Or(_) => "||",
            BinOp::BitXor(_) => "^",
            BinOp::BitAnd(_) => "&",
            BinOp::BitOr(_) => "|",
            BinOp::Shl(_) => "<<",
            BinOp::Shr(_) => ">>",
            BinOp::Eq(_) => "==",
            BinOp::Lt(_) => "<",
            BinOp::Le(_) => "<=",
            BinOp::Ne(_) => "!=",
            BinOp::Ge(_) => ">=",
            BinOp::Gt(_) => ">",
            BinOp::AddEq(_) => "+=",
            BinOp::SubEq(_) => "-=",
            BinOp::MulEq(_) => "*=",
            BinOp::DivEq(_) => "/=",
            BinOp::RemEq(_) => "%=",
            BinOp::BitXorEq(_) => "^=",
            BinOp::BitAndEq(_) => "&=",
            BinOp::BitOrEq(_) => "|=",
            BinOp::ShlEq(_) => "<<=",
            BinOp::ShrEq(_) => ">>=",
        });
    }

    fn unary_operator(&mut self, op: &UnOp) {
        self.word(match op {
            UnOp::Deref(_) => "*",
            UnOp::Not(_) => "!",
            UnOp::Neg(_) => "-",
        });
    }
}

pub fn requires_terminator(expr: &Expr) -> bool {
    // see https://github.com/rust-lang/rust/blob/2679c38fc/src/librustc_ast/util/classify.rs#L7-L25
    match expr {
        Expr::Unsafe(_)
        | Expr::Block(_)
        | Expr::If(_)
        | Expr::Match(_)
        | Expr::While(_)
        | Expr::Loop(_)
        | Expr::ForLoop(_)
        | Expr::Async(_)
        | Expr::TryBlock(_) => false,
        _ => true,
    }
}
