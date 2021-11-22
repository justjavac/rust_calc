use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNodeBuilder;
use std::iter::Peekable;

pub struct Parser<I: Iterator<Item = (SyntaxKind, String)>> {
    builder: GreenNodeBuilder<'static>,
    iter: Peekable<I>,
}

impl<I: Iterator<Item = (SyntaxKind, String)>> Parser<I> {
    pub fn new(iter: Peekable<I>) -> Self {
        Self {
            iter,
            builder: GreenNodeBuilder::new(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        while self
            .iter
            .peek()
            .map(|&(t, _)| t == SyntaxKind::WHITESPACE)
            .unwrap_or(false)
        {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }

    fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string.as_str());
        }
    }

    fn parse_val(&mut self) {
        match self.peek() {
            Some(SyntaxKind::NUMBER) => self.bump(),
            _ => {
                self.builder.start_node(SyntaxKind::ERROR.into());
                self.bump();
                self.builder.finish_node();
            }
        }
    }

    fn handle_operation(&mut self, tokens: &[SyntaxKind], next: fn(&mut Self)) {
        let checkpoint = self.builder.checkpoint();
        next(self);
        while self.peek().map(|t| tokens.contains(&t)).unwrap_or(false) {
            self.builder
                .start_node_at(checkpoint, SyntaxKind::OPERATION.into());
            self.bump();
            next(self);
            self.builder.finish_node();
        }
    }

    fn parse_mul(&mut self) {
        self.handle_operation(&[SyntaxKind::MUL, SyntaxKind::DIV], Self::parse_val)
    }

    fn parse_add(&mut self) {
        self.handle_operation(&[SyntaxKind::ADD, SyntaxKind::SUB], Self::parse_mul)
    }

    pub fn parse(mut self) -> SyntaxNode {
        self.builder.start_node(SyntaxKind::ROOT.into());
        self.parse_add();
        self.builder.finish_node();

        SyntaxNode::new_root(self.builder.finish())
    }
}
