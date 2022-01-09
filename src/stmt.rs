use crate::algorithm::Printer;
use crate::INDENT;
use syn::{Block, Expr, Local, Stmt};

impl Printer {
    pub fn block(&mut self, block: &Block) {
        self.word("{");
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => self.local(local),
            Stmt::Item(item) => self.item(item),
            Stmt::Expr(expr) => {
                self.ibox(INDENT);
                self.expr(expr);
                self.end();
                self.hardbreak();
            }
            Stmt::Semi(expr, _semi) => {
                if let Expr::Verbatim(tokens) = expr {
                    if tokens.is_empty() {
                        return;
                    }
                }
                self.ibox(INDENT);
                self.expr(expr);
                self.word(";");
                self.end();
                self.hardbreak();
            }
        }
    }

    fn local(&mut self, local: &Local) {
        self.outer_attrs(&local.attrs);
        self.ibox(INDENT);
        self.word("let ");
        self.pat(&local.pat);
        if let Some((_eq, init)) = &local.init {
            self.word(" = ");
            self.expr(init);
        }
        self.word(";");
        self.end();
        self.hardbreak();
    }
}
