use crate::algorithm::Printer;
use syn::{
    BoundLifetimes, ConstParam, GenericParam, Generics, LifetimeDef, PredicateEq,
    PredicateLifetime, PredicateType, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound,
    WhereClause, WherePredicate,
};

impl Printer {
    pub fn generics(&mut self, generics: &Generics) {
        if generics.params.is_empty() {
            return;
        }

        self.word("<");

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        //
        // TODO: ordering rules for const parameters vs type parameters have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        for param in &generics.params {
            if let GenericParam::Lifetime(_) = param {
                self.generic_param(param);
                self.word(",");
            }
        }
        for param in &generics.params {
            match param {
                GenericParam::Type(_) | GenericParam::Const(_) => {
                    self.generic_param(param);
                    self.word(",");
                }
                GenericParam::Lifetime(_) => {}
            }
        }

        self.word(">");
    }

    fn generic_param(&mut self, generic_param: &GenericParam) {
        match generic_param {
            GenericParam::Type(type_param) => self.type_param(type_param),
            GenericParam::Lifetime(lifetime_def) => self.lifetime_def(lifetime_def),
            GenericParam::Const(const_param) => self.const_param(const_param),
        }
    }

    pub fn bound_lifetimes(&mut self, bound_lifetimes: &BoundLifetimes) {
        self.word("for<");
        for (i, lifetime_def) in bound_lifetimes.lifetimes.iter().enumerate() {
            self.lifetime_def(lifetime_def);
            if i < bound_lifetimes.lifetimes.len() - 1 {
                self.word(", ");
            }
        }
        self.word("> ");
    }

    fn lifetime_def(&mut self, lifetime_def: &LifetimeDef) {
        self.outer_attrs(&lifetime_def.attrs);
        self.lifetime(&lifetime_def.lifetime);
        for (i, lifetime) in lifetime_def.bounds.iter().enumerate() {
            if i == 0 {
                self.word(":");
            } else {
                self.word("+");
            }
            self.lifetime(lifetime);
        }
    }

    fn type_param(&mut self, type_param: &TypeParam) {
        self.outer_attrs(&type_param.attrs);
        self.ident(&type_param.ident);
        for (i, type_param_bound) in type_param.bounds.iter().enumerate() {
            if i == 0 {
                self.word(":");
            } else {
                self.word("+");
            }
            self.type_param_bound(type_param_bound);
        }
        if let Some(default) = &type_param.default {
            self.word("=");
            self.ty(default);
        }
    }

    pub fn type_param_bound(&mut self, type_param_bound: &TypeParamBound) {
        match type_param_bound {
            TypeParamBound::Trait(trait_bound) => self.trait_bound(trait_bound),
            TypeParamBound::Lifetime(lifetime) => self.lifetime(lifetime),
        }
    }

    fn trait_bound(&mut self, trait_bound: &TraitBound) {
        if trait_bound.paren_token.is_some() {
            self.word("(");
        }
        let skip = match trait_bound.path.segments.first() {
            Some(segment) if segment.ident == "const" => {
                self.word("~const");
                1
            }
            _ => 0,
        };
        self.trait_bound_modifier(&trait_bound.modifier);
        if let Some(bound_lifetimes) = &trait_bound.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        for (i, segment) in trait_bound.path.segments.iter().skip(skip).enumerate() {
            if i > 0 || trait_bound.path.leading_colon.is_some() {
                self.word("::");
            }
            self.path_segment(segment);
        }
        if trait_bound.paren_token.is_some() {
            self.word(")");
        }
    }

    fn trait_bound_modifier(&mut self, trait_bound_modifier: &TraitBoundModifier) {
        match trait_bound_modifier {
            TraitBoundModifier::None => {}
            TraitBoundModifier::Maybe(_question_mark) => self.word("?"),
        }
    }

    fn const_param(&mut self, const_param: &ConstParam) {
        self.outer_attrs(&const_param.attrs);
        self.word("const");
        self.ident(&const_param.ident);
        self.word(":");
        self.ty(&const_param.ty);
        if let Some(default) = &const_param.default {
            self.word("=");
            self.expr(default);
        }
    }

    pub fn where_clause(&mut self, where_clause: &Option<WhereClause>) {
        if let Some(where_clause) = where_clause {
            if !where_clause.predicates.is_empty() {
                self.word("where");
                for predicate in &where_clause.predicates {
                    self.where_predicate(predicate);
                    self.word(",");
                }
            }
        }
    }

    fn where_predicate(&mut self, predicate: &WherePredicate) {
        match predicate {
            WherePredicate::Type(predicate) => self.predicate_type(predicate),
            WherePredicate::Lifetime(predicate) => self.predicate_lifetime(predicate),
            WherePredicate::Eq(predicate) => self.predicate_eq(predicate),
        }
    }

    fn predicate_type(&mut self, predicate: &PredicateType) {
        if let Some(bound_lifetimes) = &predicate.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        self.ty(&predicate.bounded_ty);
        self.word(":");
        for (i, type_param_bound) in predicate.bounds.iter().enumerate() {
            if i > 0 {
                self.word("+");
            }
            self.type_param_bound(type_param_bound);
        }
    }

    fn predicate_lifetime(&mut self, predicate: &PredicateLifetime) {
        self.lifetime(&predicate.lifetime);
        self.word(":");
        for (i, lifetime) in predicate.bounds.iter().enumerate() {
            if i > 0 {
                self.word("+");
            }
            self.lifetime(lifetime);
        }
    }

    fn predicate_eq(&mut self, predicate: &PredicateEq) {
        self.ty(&predicate.lhs_ty);
        self.word("=");
        self.ty(&predicate.rhs_ty);
    }
}
