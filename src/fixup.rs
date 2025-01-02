use crate::classify;
use crate::precedence::Precedence;
use syn::{
    Expr, ExprBreak, ExprRange, ExprRawAddr, ExprReference, ExprReturn, ExprUnary, ExprYield,
    ReturnType,
};

#[derive(Copy, Clone)]
pub struct FixupContext {
    previous_operator: Precedence,
    next_operator: Precedence,

    // Print expression such that it can be parsed back as a statement
    // consisting of the original expression.
    //
    // The effect of this is for binary operators in statement position to set
    // `leftmost_subexpression_in_stmt` when printing their left-hand operand.
    //
    //     (match x {}) - 1;  // match needs parens when LHS of binary operator
    //
    //     match x {};  // not when its own statement
    //
    stmt: bool,

    // This is the difference between:
    //
    //     (match x {}) - 1;  // subexpression needs parens
    //
    //     let _ = match x {} - 1;  // no parens
    //
    // There are 3 distinguishable contexts in which `print_expr` might be
    // called with the expression `$match` as its argument, where `$match`
    // represents an expression of kind `ExprKind::Match`:
    //
    //   - stmt=false leftmost_subexpression_in_stmt=false
    //
    //     Example: `let _ = $match - 1;`
    //
    //     No parentheses required.
    //
    //   - stmt=false leftmost_subexpression_in_stmt=true
    //
    //     Example: `$match - 1;`
    //
    //     Must parenthesize `($match)`, otherwise parsing back the output as a
    //     statement would terminate the statement after the closing brace of
    //     the match, parsing `-1;` as a separate statement.
    //
    //   - stmt=true leftmost_subexpression_in_stmt=false
    //
    //     Example: `$match;`
    //
    //     No parentheses required.
    leftmost_subexpression_in_stmt: bool,

    // Print expression such that it can be parsed as a match arm.
    //
    // This is almost equivalent to `stmt`, but the grammar diverges a tiny bit
    // between statements and match arms when it comes to braced macro calls.
    // Macro calls with brace delimiter terminate a statement without a
    // semicolon, but do not terminate a match-arm without comma.
    //
    //     m! {} - 1;  // two statements: a macro call followed by -1 literal
    //
    //     match () {
    //         _ => m! {} - 1,  // binary subtraction operator
    //     }
    //
    match_arm: bool,

    // This is almost equivalent to `leftmost_subexpression_in_stmt`, other than
    // for braced macro calls.
    //
    // If we have `m! {} - 1` as an expression, the leftmost subexpression
    // `m! {}` will need to be parenthesized in the statement case but not the
    // match-arm case.
    //
    //     (m! {}) - 1;  // subexpression needs parens
    //
    //     match () {
    //         _ => m! {} - 1,  // no parens
    //     }
    //
    leftmost_subexpression_in_match_arm: bool,

    // This is the difference between:
    //
    //     if let _ = (Struct {}) {}  // needs parens
    //
    //     match () {
    //         () if let _ = Struct {} => {}  // no parens
    //     }
    //
    condition: bool,

    // This is the difference between:
    //
    //     if break Struct {} == (break) {}  // needs parens
    //
    //     if break break == Struct {} {}  // no parens
    //
    rightmost_subexpression_in_condition: bool,

    // This is the difference between:
    //
    //     if break ({ x }).field + 1 {}  needs parens
    //
    //     if break 1 + { x }.field {}  // no parens
    //
    leftmost_subexpression_in_optional_operand: bool,

    // This is the difference between:
    //
    //     let _ = (return) - 1;  // without paren, this would return -1
    //
    //     let _ = return + 1;  // no paren because '+' cannot begin expr
    //
    next_operator_can_begin_expr: bool,

    // This is the difference between:
    //
    //     let _ = 1 + return 1;  // no parens if rightmost subexpression
    //
    //     let _ = 1 + (return 1) + 1;  // needs parens
    //
    next_operator_can_continue_expr: bool,

    // This is the difference between:
    //
    //     let _ = x as u8 + T;
    //
    //     let _ = (x as u8) < T;
    //
    // Without parens, the latter would want to parse `u8<T...` as a type.
    next_operator_can_begin_generics: bool,
}

impl FixupContext {
    /// The default amount of fixing is minimal fixing. Fixups should be turned
    /// on in a targeted fashion where needed.
    pub const NONE: Self = FixupContext {
        previous_operator: Precedence::MIN,
        next_operator: Precedence::MIN,
        stmt: false,
        leftmost_subexpression_in_stmt: false,
        match_arm: false,
        leftmost_subexpression_in_match_arm: false,
        condition: false,
        rightmost_subexpression_in_condition: false,
        leftmost_subexpression_in_optional_operand: false,
        next_operator_can_begin_expr: false,
        next_operator_can_continue_expr: false,
        next_operator_can_begin_generics: false,
    };

    /// Create the initial fixup for printing an expression in statement
    /// position.
    pub fn new_stmt() -> Self {
        FixupContext {
            stmt: true,
            ..FixupContext::NONE
        }
    }

    /// Create the initial fixup for printing an expression as the right-hand
    /// side of a match arm.
    pub fn new_match_arm() -> Self {
        FixupContext {
            match_arm: true,
            ..FixupContext::NONE
        }
    }

    /// Create the initial fixup for printing an expression as the "condition"
    /// of an `if` or `while`. There are a few other positions which are
    /// grammatically equivalent and also use this, such as the iterator
    /// expression in `for` and the scrutinee in `match`.
    pub fn new_condition() -> Self {
        FixupContext {
            condition: true,
            rightmost_subexpression_in_condition: true,
            ..FixupContext::NONE
        }
    }

    /// Transform this fixup into the one that should apply when printing the
    /// leftmost subexpression of the current expression.
    ///
    /// The leftmost subexpression is any subexpression that has the same first
    /// token as the current expression, but has a different last token.
    ///
    /// For example in `$a + $b` and `$a.method()`, the subexpression `$a` is a
    /// leftmost subexpression.
    ///
    /// Not every expression has a leftmost subexpression. For example neither
    /// `-$a` nor `[$a]` have one.
    pub fn leftmost_subexpression_with_operator(
        self,
        expr: &Expr,
        next_operator_can_begin_expr: bool,
        next_operator_can_begin_generics: bool,
        precedence: Precedence,
    ) -> (Precedence, Self) {
        let fixup = FixupContext {
            next_operator: precedence,
            stmt: false,
            leftmost_subexpression_in_stmt: self.stmt || self.leftmost_subexpression_in_stmt,
            match_arm: false,
            leftmost_subexpression_in_match_arm: self.match_arm
                || self.leftmost_subexpression_in_match_arm,
            rightmost_subexpression_in_condition: false,
            next_operator_can_begin_expr,
            next_operator_can_continue_expr: true,
            next_operator_can_begin_generics,
            ..self
        };

        (fixup.leftmost_subexpression_precedence(expr), fixup)
    }

    /// Transform this fixup into the one that should apply when printing a
    /// leftmost subexpression followed by a `.` or `?` token, which confer
    /// different statement boundary rules compared to other leftmost
    /// subexpressions.
    pub fn leftmost_subexpression_with_dot(self, expr: &Expr) -> (Precedence, Self) {
        let fixup = FixupContext {
            next_operator: Precedence::Unambiguous,
            stmt: self.stmt || self.leftmost_subexpression_in_stmt,
            leftmost_subexpression_in_stmt: false,
            match_arm: self.match_arm || self.leftmost_subexpression_in_match_arm,
            leftmost_subexpression_in_match_arm: false,
            rightmost_subexpression_in_condition: false,
            next_operator_can_begin_expr: false,
            next_operator_can_continue_expr: true,
            next_operator_can_begin_generics: false,
            ..self
        };

        (fixup.leftmost_subexpression_precedence(expr), fixup)
    }

    fn leftmost_subexpression_precedence(self, expr: &Expr) -> Precedence {
        if !self.next_operator_can_begin_expr || self.next_operator == Precedence::Range {
            if let Scan::Bailout = scan_right(expr, self, false, 0, 0) {
                if scan_left(expr, self) {
                    return Precedence::Unambiguous;
                }
            }
        }

        self.precedence(expr)
    }

    /// Transform this fixup into the one that should apply when printing the
    /// rightmost subexpression of the current expression.
    ///
    /// The rightmost subexpression is any subexpression that has a different
    /// first token than the current expression, but has the same last token.
    ///
    /// For example in `$a + $b` and `-$b`, the subexpression `$b` is a
    /// rightmost subexpression.
    ///
    /// Not every expression has a rightmost subexpression. For example neither
    /// `[$b]` nor `$a.f($b)` have one.
    pub fn rightmost_subexpression(
        self,
        expr: &Expr,
        precedence: Precedence,
    ) -> (Precedence, Self) {
        let fixup = self.rightmost_subexpression_fixup(false, false, precedence);
        (fixup.rightmost_subexpression_precedence(expr), fixup)
    }

    pub fn rightmost_subexpression_fixup(
        self,
        reset_allow_struct: bool,
        optional_operand: bool,
        precedence: Precedence,
    ) -> Self {
        FixupContext {
            previous_operator: precedence,
            stmt: false,
            leftmost_subexpression_in_stmt: false,
            match_arm: false,
            leftmost_subexpression_in_match_arm: false,
            condition: self.condition && !reset_allow_struct,
            leftmost_subexpression_in_optional_operand: self.condition && optional_operand,
            ..self
        }
    }

    pub fn rightmost_subexpression_precedence(self, expr: &Expr) -> Precedence {
        let default_prec = self.precedence(expr);

        if default_prec < Precedence::Prefix
            && (!self.next_operator_can_begin_expr || self.next_operator == Precedence::Range)
        {
            if let Scan::Bailout | Scan::Fail = scan_right(
                expr,
                self,
                self.previous_operator == Precedence::Range,
                1,
                0,
            ) {
                if scan_left(expr, self) {
                    return Precedence::Prefix;
                }
            }
        }

        default_prec
    }

    /// Determine whether parentheses are needed around the given expression to
    /// head off the early termination of a statement or condition.
    pub fn parenthesize(self, expr: &Expr) -> bool {
        (self.leftmost_subexpression_in_stmt && !classify::requires_semi_to_be_stmt(expr))
            || ((self.stmt || self.leftmost_subexpression_in_stmt) && matches!(expr, Expr::Let(_)))
            || (self.leftmost_subexpression_in_match_arm
                && !classify::requires_comma_to_be_match_arm(expr))
            || (self.condition && matches!(expr, Expr::Struct(_)))
            || (self.rightmost_subexpression_in_condition
                && matches!(
                    expr,
                    Expr::Return(ExprReturn { expr: None, .. })
                        | Expr::Yield(ExprYield { expr: None, .. })
                ))
            || (self.rightmost_subexpression_in_condition
                && !self.condition
                && matches!(
                    expr,
                    Expr::Break(ExprBreak { expr: None, .. })
                        | Expr::Path(_)
                        | Expr::Range(ExprRange { end: None, .. })
                ))
            || (self.leftmost_subexpression_in_optional_operand
                && matches!(expr, Expr::Block(expr) if expr.attrs.is_empty() && expr.label.is_none()))
    }

    /// Determines the effective precedence of a subexpression. Some expressions
    /// have higher or lower precedence when adjacent to particular operators.
    fn precedence(self, expr: &Expr) -> Precedence {
        if self.next_operator_can_begin_expr {
            // Decrease precedence of value-less jumps when followed by an
            // operator that would otherwise get interpreted as beginning a
            // value for the jump.
            if let Expr::Break(ExprBreak { expr: None, .. })
            | Expr::Return(ExprReturn { expr: None, .. })
            | Expr::Yield(ExprYield { expr: None, .. }) = expr
            {
                return Precedence::Jump;
            }
        }

        if !self.next_operator_can_continue_expr {
            match expr {
                // Increase precedence of expressions that extend to the end of
                // current statement or group.
                Expr::Break(_)
                | Expr::Closure(_)
                | Expr::Let(_)
                | Expr::Return(_)
                | Expr::Yield(_) => {
                    return Precedence::Prefix;
                }
                Expr::Range(e) if e.start.is_none() => return Precedence::Prefix,
                _ => {}
            }
        }

        if self.next_operator_can_begin_generics {
            if let Expr::Cast(cast) = expr {
                if classify::trailing_unparameterized_path(&cast.ty) {
                    return Precedence::MIN;
                }
            }
        }

        Precedence::of(expr)
    }
}

#[derive(Copy, Clone)]
enum Scan {
    Fail,
    Bailout,
    Consume,
}

fn scan_left(expr: &Expr, fixup: FixupContext) -> bool {
    match expr {
        Expr::Assign(_) => fixup.previous_operator <= Precedence::Assign,
        Expr::Binary(e) => match Precedence::of_binop(&e.op) {
            Precedence::Assign => fixup.previous_operator <= Precedence::Assign,
            binop_prec => fixup.previous_operator < binop_prec,
        },
        Expr::Range(e) => e.start.is_none() || fixup.previous_operator < Precedence::Assign,
        _ => true,
    }
}

fn scan_right(
    expr: &Expr,
    fixup: FixupContext,
    range: bool,
    fail_offset: u8,
    bailout_offset: u8,
) -> Scan {
    if fixup.parenthesize(expr) {
        return Scan::Consume;
    }
    match expr {
        #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
        Expr::Assign(e) => {
            if match fixup.next_operator {
                Precedence::Unambiguous => fail_offset >= 2,
                _ => bailout_offset >= 1,
            } {
                return Scan::Consume;
            }
            let right_fixup = fixup.rightmost_subexpression_fixup(false, false, Precedence::Assign);
            let scan = scan_right(
                &e.right,
                right_fixup,
                false,
                match fixup.next_operator {
                    Precedence::Unambiguous => fail_offset,
                    _ => 1,
                },
                1,
            );
            if let Scan::Bailout | Scan::Consume = scan {
                return Scan::Consume;
            }
            if right_fixup.rightmost_subexpression_precedence(&e.right) < Precedence::Assign {
                Scan::Consume
            } else if let Precedence::Unambiguous = fixup.next_operator {
                Scan::Fail
            } else {
                Scan::Bailout
            }
        }
        Expr::Binary(e) => {
            if match fixup.next_operator {
                Precedence::Unambiguous => fail_offset >= 2,
                _ => bailout_offset >= 1,
            } {
                return Scan::Consume;
            }
            let binop_prec = Precedence::of_binop(&e.op);
            let right_fixup = fixup.rightmost_subexpression_fixup(false, false, binop_prec);
            let scan = scan_right(
                &e.right,
                right_fixup,
                range && binop_prec != Precedence::Assign,
                match fixup.next_operator {
                    Precedence::Unambiguous => fail_offset,
                    _ => 1,
                },
                match (binop_prec, fixup.next_operator) {
                    (Precedence::Assign, _) => 1,
                    (_, Precedence::Assign | Precedence::Range) if range => 0,
                    _ => 1,
                },
            );
            if match (scan, fixup.next_operator) {
                (Scan::Fail, _) => false,
                (Scan::Bailout, _) if binop_prec == Precedence::Assign => true,
                (Scan::Bailout, Precedence::Assign | Precedence::Range) => !range,
                (Scan::Bailout | Scan::Consume, _) => true,
            } {
                return Scan::Consume;
            }
            let right_prec = right_fixup.rightmost_subexpression_precedence(&e.right);
            let right_needs_group = match binop_prec {
                Precedence::Assign => right_prec < binop_prec,
                _ => right_prec <= binop_prec,
            };
            if right_needs_group {
                Scan::Consume
            } else if let (Scan::Fail, Precedence::Unambiguous) = (scan, fixup.next_operator) {
                Scan::Fail
            } else {
                Scan::Bailout
            }
        }
        Expr::RawAddr(ExprRawAddr { expr, .. })
        | Expr::Reference(ExprReference { expr, .. })
        | Expr::Unary(ExprUnary { expr, .. }) => {
            if match fixup.next_operator {
                Precedence::Unambiguous => fail_offset >= 2,
                _ => bailout_offset >= 1,
            } {
                return Scan::Consume;
            }
            let right_fixup = fixup.rightmost_subexpression_fixup(false, false, Precedence::Prefix);
            let scan = scan_right(
                expr,
                right_fixup,
                range,
                match fixup.next_operator {
                    Precedence::Unambiguous => fail_offset,
                    _ => 1,
                },
                match fixup.next_operator {
                    Precedence::Assign | Precedence::Range if range => 0,
                    _ => 1,
                },
            );
            if match (scan, fixup.next_operator) {
                (Scan::Fail, _) => false,
                (Scan::Bailout, Precedence::Assign | Precedence::Range) => !range,
                (Scan::Bailout | Scan::Consume, _) => true,
            } {
                return Scan::Consume;
            }
            if right_fixup.rightmost_subexpression_precedence(expr) < Precedence::Prefix {
                Scan::Consume
            } else if let (Scan::Fail, Precedence::Unambiguous) = (scan, fixup.next_operator) {
                Scan::Fail
            } else {
                Scan::Bailout
            }
        }
        Expr::Range(e) => match &e.end {
            Some(end) => {
                if fail_offset >= 2 {
                    return Scan::Consume;
                }
                let right_fixup =
                    fixup.rightmost_subexpression_fixup(false, true, Precedence::Range);
                let scan = scan_right(
                    end,
                    right_fixup,
                    true,
                    fail_offset,
                    match fixup.next_operator {
                        Precedence::Assign | Precedence::Range => 0,
                        _ => 1,
                    },
                );
                if match (scan, fixup.next_operator) {
                    (Scan::Fail, _) => false,
                    (Scan::Bailout, Precedence::Assign | Precedence::Range) => false,
                    (Scan::Bailout | Scan::Consume, _) => true,
                } {
                    return Scan::Consume;
                }
                if right_fixup.rightmost_subexpression_precedence(end) <= Precedence::Range {
                    Scan::Consume
                } else {
                    Scan::Fail
                }
            }
            None => match fixup.next_operator {
                Precedence::Range => Scan::Consume,
                _ => Scan::Fail,
            },
        },
        Expr::Break(e) => match &e.expr {
            Some(value) => {
                if bailout_offset >= 1 || e.label.is_none() && classify::expr_leading_label(value) {
                    return Scan::Consume;
                }
                let right_fixup = fixup.rightmost_subexpression_fixup(true, true, Precedence::Jump);
                match scan_right(value, right_fixup, false, 1, 1) {
                    Scan::Fail => Scan::Bailout,
                    Scan::Bailout | Scan::Consume => Scan::Consume,
                }
            }
            None => match fixup.next_operator {
                Precedence::Assign if range => Scan::Fail,
                _ => Scan::Consume,
            },
        },
        Expr::Return(ExprReturn { expr, .. }) | Expr::Yield(ExprYield { expr, .. }) => match expr {
            Some(e) => {
                if bailout_offset >= 1 {
                    return Scan::Consume;
                }
                let right_fixup =
                    fixup.rightmost_subexpression_fixup(true, false, Precedence::Jump);
                match scan_right(e, right_fixup, false, 1, 1) {
                    Scan::Fail => Scan::Bailout,
                    Scan::Bailout | Scan::Consume => Scan::Consume,
                }
            }
            None => match fixup.next_operator {
                Precedence::Assign if range => Scan::Fail,
                _ => Scan::Consume,
            },
        },
        // false positive: https://github.com/rust-lang/rust/issues/135137
        #[cfg_attr(all(test, exhaustive), allow(non_exhaustive_omitted_patterns))]
        Expr::Closure(e) => {
            if matches!(e.output, ReturnType::Default)
                || matches!(&*e.body, Expr::Block(body) if body.attrs.is_empty() && body.label.is_none())
            {
                if bailout_offset >= 1 {
                    return Scan::Consume;
                }
                let right_fixup =
                    fixup.rightmost_subexpression_fixup(false, false, Precedence::Jump);
                match scan_right(&e.body, right_fixup, false, 1, 1) {
                    Scan::Fail => Scan::Bailout,
                    Scan::Bailout | Scan::Consume => Scan::Consume,
                }
            } else {
                Scan::Consume
            }
        }
        Expr::Group(e) => scan_right(&e.expr, fixup, range, fail_offset, bailout_offset),
        Expr::Array(_)
        | Expr::Async(_)
        | Expr::Await(_)
        | Expr::Block(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Const(_)
        | Expr::Continue(_)
        | Expr::Field(_)
        | Expr::ForLoop(_)
        | Expr::If(_)
        | Expr::Index(_)
        | Expr::Infer(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Loop(_)
        | Expr::Macro(_)
        | Expr::Match(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Repeat(_)
        | Expr::Struct(_)
        | Expr::Try(_)
        | Expr::TryBlock(_)
        | Expr::Tuple(_)
        | Expr::Unsafe(_)
        | Expr::Verbatim(_)
        | Expr::While(_) => match fixup.next_operator {
            Precedence::Assign | Precedence::Range if range => Scan::Fail,
            _ => Scan::Consume,
        },

        _ => match fixup.next_operator {
            Precedence::Assign | Precedence::Range if range => Scan::Fail,
            _ => Scan::Consume,
        },
    }
}
