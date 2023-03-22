use crate::algorithm::Printer;
use syn::{Expr, Stmt};

impl Printer {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => {
                self.outer_attrs(&local.attrs);
                self.ibox(0);
                self.word("let ");
                self.pat(&local.pat);
                if let Some((_eq, init)) = &local.init {
                    self.word(" = ");
                    self.neverbreak();
                    self.expr(init);
                }
                self.word(";");
                self.end();
                self.hardbreak();
            }
            Stmt::Item(item) => self.item(item),
            Stmt::Expr(expr) => {
                if break_after(expr) {
                    self.ibox(0);
                    self.expr_beginning_of_line(expr, true);
                    if add_semi(expr) {
                        self.word(";");
                    }
                    self.end();
                    self.hardbreak();
                } else {
                    self.expr_beginning_of_line(expr, true);
                }
            }
            Stmt::Semi(expr, _semi) => {
                if let Expr::Verbatim(tokens) = expr {
                    if tokens.is_empty() {
                        return;
                    }
                }
                self.ibox(0);
                self.expr_beginning_of_line(expr, true);
                if !remove_semi(expr) {
                    self.word(";");
                }
                self.end();
                self.hardbreak();
            }
        }
    }
}

pub fn add_semi(expr: &Expr) -> bool {
    match expr {
        Expr::Assign(_)
        | Expr::AssignOp(_)
        | Expr::Break(_)
        | Expr::Continue(_)
        | Expr::Return(_)
        | Expr::Yield(_) => true,
        Expr::Group(group) => add_semi(&group.expr),

        Expr::Array(_)
        | Expr::Async(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Block(_)
        | Expr::Box(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Closure(_)
        | Expr::Field(_)
        | Expr::ForLoop(_)
        | Expr::If(_)
        | Expr::Index(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Loop(_)
        | Expr::Macro(_)
        | Expr::Match(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Range(_)
        | Expr::Reference(_)
        | Expr::Repeat(_)
        | Expr::Struct(_)
        | Expr::Try(_)
        | Expr::TryBlock(_)
        | Expr::Tuple(_)
        | Expr::Type(_)
        | Expr::Unary(_)
        | Expr::Unsafe(_)
        | Expr::Verbatim(_)
        | Expr::While(_) => false,

        #[cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
        _ => false,
    }
}

pub fn break_after(expr: &Expr) -> bool {
    if let Expr::Group(group) = expr {
        if let Expr::Verbatim(verbatim) = group.expr.as_ref() {
            return !verbatim.is_empty();
        }
    }
    true
}

fn remove_semi(expr: &Expr) -> bool {
    match expr {
        Expr::ForLoop(_) | Expr::While(_) => true,
        Expr::Group(group) => remove_semi(&group.expr),
        Expr::If(expr) => match &expr.else_branch {
            Some((_else_token, else_branch)) => remove_semi(else_branch),
            None => true,
        },

        Expr::Array(_)
        | Expr::Assign(_)
        | Expr::AssignOp(_)
        | Expr::Async(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Block(_)
        | Expr::Box(_)
        | Expr::Break(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Closure(_)
        | Expr::Continue(_)
        | Expr::Field(_)
        | Expr::Index(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Loop(_)
        | Expr::Macro(_)
        | Expr::Match(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Range(_)
        | Expr::Reference(_)
        | Expr::Repeat(_)
        | Expr::Return(_)
        | Expr::Struct(_)
        | Expr::Try(_)
        | Expr::TryBlock(_)
        | Expr::Tuple(_)
        | Expr::Type(_)
        | Expr::Unary(_)
        | Expr::Unsafe(_)
        | Expr::Verbatim(_)
        | Expr::Yield(_) => false,

        #[cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
        _ => false,
    }
}
