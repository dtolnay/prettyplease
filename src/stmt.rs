use crate::unparse::Printer;
use syn::{Block, Local, Stmt};

impl Printer {
    pub fn block(&mut self, block: &Block) {
        self.word("{");
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
        self.word("}");
    }

    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => self.local(local),
            Stmt::Item(item) => self.item(item),
            Stmt::Expr(expr) => self.expr(expr),
            Stmt::Semi(expr, _semi) => {
                self.expr(expr);
                self.word(";");
            }
        }
    }

    fn local(&mut self, local: &Local) {
        self.outer_attrs(&local.attrs);
        self.word("let");
        self.pat(&local.pat);
        if let Some((_eq, init)) = &local.init {
            self.word("=");
            self.expr(init);
        }
        self.word(";");
    }
}
