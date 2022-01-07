use crate::algorithm::Printer;
use crate::INDENT;
use syn::{Block, Local, Stmt};

impl Printer {
    pub fn block(&mut self, block: &Block) {
        self.word("{");
        if !block.stmts.is_empty() {
            self.cbox(INDENT);
            self.hardbreak();
            for stmt in &block.stmts {
                self.stmt(stmt);
            }
            self.offset(-INDENT);
            self.end();
        }
        self.word("}");
    }

    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => self.local(local),
            Stmt::Item(item) => self.item(item),
            Stmt::Expr(expr) => {
                self.expr(expr);
                self.hardbreak();
            }
            Stmt::Semi(expr, _semi) => {
                self.expr(expr);
                self.word(";");
                self.hardbreak();
            }
        }
    }

    fn local(&mut self, local: &Local) {
        self.outer_attrs(&local.attrs);
        self.word("let ");
        self.pat(&local.pat);
        if let Some((_eq, init)) = &local.init {
            self.word(" = ");
            self.expr(init);
        }
        self.word(";");
        self.hardbreak();
    }
}
