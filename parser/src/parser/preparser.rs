//! Groups tokens into token AST.

use proc_macro2::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};
use std::collections::VecDeque;
use std::mem;
use syn;

use crate::ast::common;

pub(super) mod ast {

    use crate::ast::common;
    use crate::ast::ids::ExprId;
    use proc_macro2::{TokenStream, TokenTree};

    pub type ExprType = TokenStream;

    pub type Assertion = common::Assertion<ExprType>;
    pub type Expression = common::Expression<ExprType>;
    pub type Conjunction = common::Conjunction<ExprType>;

    impl Expression {
        pub fn new(expr: Vec<TokenTree>) -> Self {
            assert!(!expr.is_empty());
            Self {
                id: ExprId::new(),
                body: expr.into_iter().collect(),
            }
        }
    }
}

pub fn preparse_assertion(tokens: TokenStream) -> syn::Result<ast::Assertion> {
    let mut parser = Parser::new(tokens);
    let assertion = parser.extract_assertion()?;
    assert!(parser.is_finished());
    Ok(assertion)
}

struct ParserStream {
    last_span: Span,
    tokens: VecDeque<TokenTree>,
}

impl ParserStream {
    fn empty() -> Self {
        Self {
            tokens: VecDeque::new(),
            last_span: Span::call_site(),
        }
    }
    fn from_token_stream(tokens: TokenStream) -> Self {
        let token_queue: VecDeque<_> = tokens.into_iter().collect();
        Self {
            tokens: token_queue,
            last_span: Span::call_site(),
        }
    }
    fn last_span(&self) -> Span {
        self.last_span
    }
    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    fn pop(&mut self) -> Option<TokenTree> {
        if let Some(token) = self.tokens.pop_front() {
            self.last_span = token.span();
            Some(token)
        } else {
            None
        }
    }
    /// Check if the input starts with the keyword and if yes consume it.
    fn check_keyword(&mut self, keyword: &str) -> bool {
        if let Some(TokenTree::Ident(ident)) = self.tokens.front() {
            if ident.to_string() == keyword {
                self.pop();
                return true;
            }
        }
        false
    }
    /// Check if the input starts with the operator.
    fn peek_operator(&self, operator: &str) -> bool {
        for (i, c) in operator.char_indices() {
            if let Some(TokenTree::Punct(punct)) = self.tokens.get(i) {
                if punct.as_char() != c {
                    return false;
                }
                if i + 1 < operator.len() && punct.spacing() != Spacing::Joint {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    /// Check whether the input starts an operator.
    fn peek_any_operator(&self) -> bool {
        self.peek_operator("==>") || self.peek_operator("&&")
    }
    /// Check if the input starts with the operator and if yes consume it.
    fn check_operator(&mut self, operator: &str) -> bool {
        if !self.peek_operator(operator) {
            return false;
        }
        for _ in operator.chars() {
            self.pop();
        }
        true
    }
    /// Check if we have a nested assertion here.
    fn check_nested_assertion(&mut self) -> Option<Group> {
        if let Some(TokenTree::Group(group)) = self.tokens.front() {
            if group.delimiter() == Delimiter::Parenthesis {
                if let Some(TokenTree::Group(group)) = self.pop() {
                    return Some(group);
                } else {
                    unreachable!();
                }
            }
        }
        None
    }
}

struct Parser {
    input: ParserStream,
    conjuncts: Vec<ast::Assertion>,
    expr: Vec<TokenTree>,
    consumed_expression: bool,
}

impl Parser {
    fn new(tokens: TokenStream) -> Self {
        let input = ParserStream::from_token_stream(tokens);
        Self {
            input: input,
            conjuncts: Vec::new(),
            expr: Vec::new(),
            consumed_expression: false,
        }
    }
    fn is_finished(&self) -> bool {
        self.input.is_empty()
    }
    fn new_expr(&mut self) -> ast::Expression {
        let expr = mem::replace(&mut self.expr, Vec::new());
        ast::Expression::new(expr)
    }
    fn convert_expr_into_conjunct(&mut self) -> syn::Result<()> {
        if !self.expr.is_empty() {
            let expression = self.new_expr();
            self.conjuncts
                .push(common::Assertion::Expression(expression));
            Ok(())
        } else {
            if !self.consumed_expression {
                Err(self.missing_assertion_error())
            } else {
                self.consumed_expression = false;
                Ok(())
            }
        }
    }
    /// Traverse the tokens and reconstruct the high-level structure of the assertion.
    ///
    /// Grammar (note that the first match wins):
    ///
    /// ```plain
    /// assertion :=
    ///     { conjunct '&&' } forall { '&&' conjunct } |
    ///     implication |
    ///     conjunct { '&&' conjunct }
    /// forall = 'forall' '|' args '|' assertion 'triggers' '{' rust_expression '}'
    /// implication = conjunct { '&&' conjunct } '==>' conjunct { '&&' conjunct }
    /// conjunct := '(' assertion ')' | '{' assertion '}' | '[' assertion ']' | pledge
    /// pledge := 'after_expiry' [ '<' identifier '>' ] '(' assertion ')'
    /// conjunct := rust_expression     // if failed to match any of the above, treat as
    ///                                 // a Rust expression
    /// ```
    fn extract_assertion(&mut self) -> syn::Result<ast::Assertion> {
        while !self.input.is_empty() {
            if self.input.check_keyword("forall") {
                unimplemented!("check_forall_keyword");
            } else if self.input.check_keyword("after_expiry") {
                unimplemented!("after_expiry_keyword");
            } else if self.input.check_operator("==>") {
                self.convert_expr_into_conjunct()?;
                let mut parser = Parser {
                    input: mem::replace(&mut self.input, ParserStream::empty()),
                    conjuncts: Vec::new(),
                    expr: Vec::new(),
                    consumed_expression: false,
                };
                let lhs = self.conjuncts_to_assertion()?;
                let rhs = parser.extract_assertion()?;
                assert!(parser.is_finished());
                let implication = common::Implication {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
                return Ok(common::Assertion::Implication(implication));
            } else if self.input.check_operator("&&") {
                self.convert_expr_into_conjunct()?;
                if self.input.is_empty() {
                    return Err(self.missing_assertion_error());
                }
            } else if let Some(group) = self.input.check_nested_assertion() {
                if self.expr.is_empty() && (self.input.is_empty() || self.input.peek_any_operator())
                {
                    // We potentially have a nested expression.
                    let mut parser = Parser::new(group.stream());
                    let conjunct = parser.extract_assertion()?;
                    assert!(parser.is_finished());
                    if let common::Assertion::Expression(expression) = conjunct {
                        self.expr.extend(expression.body)
                    } else {
                        self.conjuncts.push(conjunct);
                        self.consumed_expression = true;
                    }
                } else {
                    self.expr.push(TokenTree::Group(group));
                }
            } else {
                let token = self.input.pop().unwrap();
                self.expr.push(token);
            }
        }
        if !self.expr.is_empty() {
            let expression = self.new_expr();
            self.conjuncts
                .push(common::Assertion::Expression(expression));
        }
        self.conjuncts_to_assertion()
    }

    fn conjuncts_to_assertion(&mut self) -> syn::Result<ast::Assertion> {
        if self.conjuncts.is_empty() {
            Err(self.missing_assertion_error())
        } else {
            let assertion = if self.conjuncts.len() == 1 {
                self.conjuncts.pop().unwrap()
            } else {
                let conjunction = ast::Conjunction {
                    conjuncts: mem::replace(&mut self.conjuncts, Vec::new()),
                };
                common::Assertion::Conjunction(conjunction)
            };
            Ok(assertion)
        }
    }

    fn missing_assertion_error(&self) -> syn::Error {
        syn::Error::new(self.input.last_span(), "missing assertion")
    }
}
