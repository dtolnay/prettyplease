use crate::algorithm::Printer;
use crate::iter::IterDelimited;
use crate::INDENT;
use std::ptr;
use syn::{
    AngleBracketedGenericArguments, AssocConst, AssocType, Constraint, Expr, GenericArgument,
    ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
};

#[derive(Copy, Clone, PartialEq)]
pub enum PathKind {
    // a::B
    Simple,
    // a::B<T>
    Type,
    // a::B::<T>
    Expr,
}

impl Printer {
    pub fn path(&mut self, path: &Path, kind: PathKind) {
        assert!(!path.segments.is_empty());
        for segment in path.segments.iter().delimited() {
            if !segment.is_first || path.leading_colon.is_some() {
                self.word("::");
            }
            self.path_segment(&segment, kind);
        }
    }

    pub fn path_segment(&mut self, segment: &PathSegment, kind: PathKind) {
        self.ident(&segment.ident);
        self.path_arguments(&segment.arguments, kind);
    }

    fn path_arguments(&mut self, arguments: &PathArguments, kind: PathKind) {
        match arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(arguments) => {
                self.angle_bracketed_generic_arguments(arguments, kind);
            }
            PathArguments::Parenthesized(arguments) => {
                self.parenthesized_generic_arguments(arguments);
            }
        }
    }

    pub fn generic_argument(&mut self, arg: &GenericArgument) {
        match arg {
            GenericArgument::Lifetime(lifetime) => self.lifetime(lifetime),
            GenericArgument::Type(ty) => self.ty(ty),
            GenericArgument::AssocConst(assoc_const) => self.assoc_const(assoc_const),
            GenericArgument::AssocType(assoc_type) => self.assoc_type(assoc_type),
            GenericArgument::Constraint(constraint) => self.constraint(constraint),
            GenericArgument::Const(expr) => {
                match expr {
                    Expr::Lit(expr) => self.expr_lit(expr),
                    Expr::Block(expr) => self.expr_block(expr),
                    // ERROR CORRECTION: Add braces to make sure that the
                    // generated code is valid.
                    _ => {
                        self.word("{");
                        self.expr(expr);
                        self.word("}");
                    }
                }
            }
            _ => unreachable!("unknown GenericArgument"),
        }
    }

    pub fn angle_bracketed_generic_arguments(
        &mut self,
        generic: &AngleBracketedGenericArguments,
        path_kind: PathKind,
    ) {
        if generic.args.is_empty() || path_kind == PathKind::Simple {
            return;
        }

        if path_kind == PathKind::Expr {
            self.word("::");
        }
        self.word("<");
        self.cbox(INDENT);
        self.zerobreak();

        // Print lifetimes before types and consts, all before bindings,
        // regardless of their order in self.args.
        //
        // TODO: ordering rules for const arguments vs type arguments have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        enum Group {
            First,
            Second,
            Third,
        }
        fn group(arg: &GenericArgument) -> Group {
            match arg {
                GenericArgument::Lifetime(_) => Group::First,
                GenericArgument::Type(_) | GenericArgument::Const(_) => Group::Second,
                GenericArgument::AssocConst(_)
                | GenericArgument::AssocType(_)
                | GenericArgument::Constraint(_) => Group::Third,
                _ => unimplemented!("unknown GenericArgument"),
            }
        }
        let last = generic.args.iter().max_by_key(|param| group(param));
        for current_group in [Group::First, Group::Second, Group::Third] {
            for arg in &generic.args {
                if group(arg) == current_group {
                    self.generic_argument(arg);
                    self.trailing_comma(ptr::eq(arg, last.unwrap()));
                }
            }
        }

        self.offset(-INDENT);
        self.end();
        self.word(">");
    }

    fn assoc_type(&mut self, binding: &AssocType) {
        self.ident(&binding.ident);
        self.word(" = ");
        self.ty(&binding.ty);
    }

    fn assoc_const(&mut self, binding: &AssocConst) {
        self.ident(&binding.ident);
        self.word(" : ");
        if let Some(g) = &binding.generics {
            self.angle_bracketed_generic_arguments(g, PathKind::Type);
        }
        self.word(" = ");
        self.expr(&binding.value);
    }

    fn constraint(&mut self, constraint: &Constraint) {
        self.ident(&constraint.ident);
        self.ibox(INDENT);
        for bound in constraint.bounds.iter().delimited() {
            if bound.is_first {
                self.word(": ");
            } else {
                self.space();
                self.word("+ ");
            }
            self.type_param_bound(&bound);
        }
        self.end();
    }

    fn parenthesized_generic_arguments(&mut self, arguments: &ParenthesizedGenericArguments) {
        self.cbox(INDENT);
        self.word("(");
        self.zerobreak();
        for ty in arguments.inputs.iter().delimited() {
            self.ty(&ty);
            self.trailing_comma(ty.is_last);
        }
        self.offset(-INDENT);
        self.word(")");
        self.return_type(&arguments.output);
        self.end();
    }

    pub fn qpath(&mut self, qself: &Option<QSelf>, path: &Path, kind: PathKind) {
        let qself = match qself {
            Some(qself) => qself,
            None => {
                self.path(path, kind);
                return;
            }
        };

        assert!(qself.position < path.segments.len());

        self.word("<");
        self.ty(&qself.ty);

        let mut segments = path.segments.iter();
        if qself.position > 0 {
            self.word(" as ");
            for segment in segments.by_ref().take(qself.position).delimited() {
                if !segment.is_first || path.leading_colon.is_some() {
                    self.word("::");
                }
                self.path_segment(&segment, kind);
                if segment.is_last {
                    self.word(">");
                }
            }
        } else {
            self.word(">");
        }
        for segment in segments {
            self.word("::");
            self.path_segment(segment, kind);
        }
    }
}
